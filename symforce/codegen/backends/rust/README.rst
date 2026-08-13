***THIS MODULE IS EXPERIMENTAL***

Backend for Rust. The default ``nalgebra`` target supports vector and matrix
inputs and outputs. The ``stack-algebra`` target additionally supports the
geometry package (``Rot2``, ``Pose2``, ``Rot3``, ``Pose3``, and
``LinearCameraCal``, ``ATANCameraCal``, ``PolynomialCameraCal``, and ``DoubleSphereCameraCal``).  The runtime also provides a generic
``CameraCal`` trait
and ``PosedCamera`` wrapper, provided by the unified ``symforce-rust`` crate;
full camera package generation for additional calibration models is not
implemented yet.
