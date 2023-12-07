# Quantum Algorithms

Various quantum algorithms implemented in [Pennylane](https://pennylane.ai/) and [RustQIP](https://github.com/Renmusxd/RustQIP).

## Implemented Algorithms

- If you want to see each examples then click the check mark in the table below.

- If you want to see explanation of each algorithms then click the pennylane link in the table below.

|          Topic          |                      Pennylane                       |                    RustQIP                    |                           Qiskit                            |
|:-----------------------:|:----------------------------------------------------:|:---------------------------------------------:|:-----------------------------------------------------------:|
|        SWAP Test        |   [:heavy_check_mark:](./notebook/swap_test.ipynb)   | [:heavy_check_mark:](./src/bin/swap_test.rs)  |                             :x:                             |
|    Superdense Coding    |  [:heavy_check_mark:](./notebook/superdense.ipynb)   | [:heavy_check_mark:](./src/bin/superdense.rs) |                             :x:                             |
|  Quantum Teleportation  |   [:heavy_check_mark:](./notebook/teleport.ipynb)    |  [:heavy_check_mark:](./src/bin/teleport.rs)  |                             :x:                             |
|    Deutsch Algorithm    |    [:heavy_check_mark:](./notebook/deutsch.ipynb)    |  [:heavy_check_mark:](./src/bin/deutsch.rs)   |                             :x:                             |
| Deutsch-Jozsa Algorithm | [:heavy_check_mark:](./notebook/deutsch_jozsa.ipynb) |       [:x:](./src/bin/deutsch_jozsa.rs)       | [:heavy_check_mark:](./notebook_qiskit/deutsch_jozsa.ipynb) |
