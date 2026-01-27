import 'package:flutter/material.dart';
import 'package:fl_chart/fl_chart.dart';

final class BenchMetricGroup {
  const BenchMetricGroup({required this.label, required this.median, required this.mean, required this.p95});

  final String label;
  final Duration median;
  final Duration mean;
  final Duration p95;
}

final class BenchGroupedBarChart extends StatelessWidget {
  const BenchGroupedBarChart({super.key, required this.groups});

  final List<BenchMetricGroup> groups;

  @override
  Widget build(BuildContext context) {
    if (groups.isEmpty) return const SizedBox.shrink();
    final theme = Theme.of(context);
    final colors = theme.colorScheme;

    final maxMicros = groups
        .expand((g) => [g.median.inMicroseconds, g.mean.inMicroseconds, g.p95.inMicroseconds])
        .fold<int>(0, (a, b) => a > b ? a : b)
        .toDouble();
    final safeMax = maxMicros <= 0 ? 1.0 : maxMicros;
    final interval = _niceInterval(safeMax);

    final medianColor = colors.primary;
    final meanColor = colors.secondary;
    final p95Color = colors.tertiary;

    return LayoutBuilder(
      builder: (context, constraints) {
        final groupTargetWidth = 156.0;
        final minWidth = (groups.length * groupTargetWidth).clamp(520.0, double.infinity).toDouble();
        final width = constraints.maxWidth > minWidth ? constraints.maxWidth : minWidth;
        final perGroup = width / groups.length;
        final groupSpace = ((perGroup * 0.18).clamp(14.0, 44.0)).toDouble();
        final barsSpace = ((perGroup * 0.03).clamp(3.0, 8.0)).toDouble();
        final rodWidth = ((perGroup * 0.08).clamp(12.0, 20.0)).toDouble();

        final barGroups = <BarChartGroupData>[
          for (var i = 0; i < groups.length; i++)
            BarChartGroupData(
              x: i,
              barsSpace: barsSpace,
              barRods: [
                BarChartRodData(
                  toY: groups[i].median.inMicroseconds.toDouble(),
                  width: rodWidth,
                  color: medianColor,
                  borderRadius: BorderRadius.circular(4),
                ),
                BarChartRodData(
                  toY: groups[i].mean.inMicroseconds.toDouble(),
                  width: rodWidth,
                  color: meanColor,
                  borderRadius: BorderRadius.circular(4),
                ),
                BarChartRodData(
                  toY: groups[i].p95.inMicroseconds.toDouble(),
                  width: rodWidth,
                  color: p95Color,
                  borderRadius: BorderRadius.circular(4),
                ),
              ],
            ),
        ];

        return SingleChildScrollView(
          scrollDirection: Axis.horizontal,
          child: DecoratedBox(
            decoration: BoxDecoration(color: colors.surfaceContainerHighest.withAlpha(80), borderRadius: BorderRadius.circular(12)),
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
              child: SizedBox(
                width: width,
                height: 340,
                child: BarChart(
                  BarChartData(
                    alignment: BarChartAlignment.center,
                    groupsSpace: groupSpace,
                    maxY: safeMax * 1.1,
                    barGroups: barGroups,
                    gridData: FlGridData(
                      show: true,
                      horizontalInterval: interval,
                      verticalInterval: 1,
                      drawVerticalLine: true,
                      getDrawingHorizontalLine: (value) => FlLine(color: colors.outlineVariant, strokeWidth: 2),
                      getDrawingVerticalLine: (value) => FlLine(color: colors.outlineVariant, strokeWidth: 2),
                    ),
                    borderData: FlBorderData(
                      show: true,
                      border: Border(
                        left: BorderSide(color: colors.outlineVariant.withAlpha(170)),
                        bottom: BorderSide(color: colors.outlineVariant.withAlpha(170)),
                      ),
                    ),
                    barTouchData: BarTouchData(
                      enabled: true,
                      touchTooltipData: BarTouchTooltipData(
                        getTooltipItem: (group, groupIndex, rod, rodIndex) {
                          final g = groups[group.x.toInt()];
                          final metric = switch (rodIndex) {
                            0 => 'Median',
                            1 => 'Mean',
                            _ => 'P95',
                          };
                          return BarTooltipItem('${g.label}\n$metric: ${_fmtMicros(rod.toY)}', TextStyle(color: colors.onSurface));
                        },
                      ),
                    ),
                    titlesData: FlTitlesData(
                      topTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
                      rightTitles: const AxisTitles(sideTitles: SideTitles(showTitles: false)),
                      leftTitles: AxisTitles(
                        sideTitles: SideTitles(
                          showTitles: true,
                          interval: interval,
                          reservedSize: 72,
                          getTitlesWidget: (value, meta) {
                            return Text(_fmtMicros(value), style: TextStyle(color: colors.onSurfaceVariant));
                          },
                        ),
                      ),
                      bottomTitles: AxisTitles(
                        sideTitles: SideTitles(
                          showTitles: true,
                          reservedSize: 56,
                          getTitlesWidget: (value, meta) {
                            final index = value.toInt();
                            if (index < 0 || index >= groups.length) return const SizedBox.shrink();
                            return SideTitleWidget(
                              axisSide: meta.axisSide,
                              space: 10,
                              child: Text(
                                _axisLabel(groups[index].label),
                                textAlign: TextAlign.center,
                                style: TextStyle(color: colors.onSurfaceVariant),
                              ),
                            );
                          },
                        ),
                      ),
                    ),
                  ),
                ),
              ),
            ),
          ),
        );
      },
    );
  }
}

double _niceInterval(double maxMicros) {
  if (maxMicros <= 0) return 1.0;
  final raw = maxMicros / 5;
  final base = _pow10(raw);
  final normalized = raw / base;
  final step = normalized <= 1 ? 1 : (normalized <= 2 ? 2 : (normalized <= 2.5 ? 2.5 : (normalized <= 5 ? 5 : 10)));
  return (step * base).clamp(1.0, double.infinity);
}

String _fmtMicros(double micros) {
  final rounded = micros.round();
  if (rounded >= 1000) return '${(rounded / 1000).toStringAsFixed(1)} ms';
  return '$rounded µs';
}

double _pow10(double v) {
  var x = v.abs();
  if (x == 0) return 1.0;
  var p = 1.0;
  while (x >= 10) {
    x /= 10;
    p *= 10;
  }
  while (x < 1) {
    x *= 10;
    p /= 10;
  }
  return p;
}

String _axisLabel(String label) {
  if (label.startsWith('Dart ')) {
    return 'Dart\n${label.substring(5)}';
  }
  if (label.startsWith('Rust/Oxide ')) {
    return 'Rust\n${label.substring(11)}';
  }
  return label.replaceAll(' ', '\n');
}
