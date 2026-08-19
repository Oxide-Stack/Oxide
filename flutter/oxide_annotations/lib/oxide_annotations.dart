/// Annotation package consumed by `oxide_generator`.
library oxide_annotations;

// Keep this package dependency-light. It is the build-time contract between
// user code, the generator, and the generated output.
export 'src/annotations.dart';
export 'src/oxide_view.dart';
