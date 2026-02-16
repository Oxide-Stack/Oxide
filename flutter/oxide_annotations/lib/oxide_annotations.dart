/// Annotations used to generate Oxide store bindings in Dart.
///
/// This package is consumed by `oxide_generator` during build-time code
/// generation. It contains the `@OxideStore` annotation and a few helper types
/// used by the generated output.
library oxide_annotations;

// Maintenance: keep this package dependency-light and stable. It is a build-time
// contract between user code, the generator, and the generated output.
export 'src/annotations.dart';
export 'src/oxide_view.dart';
