# Dialog Rendering

This example illustrates how to organize a GUI framework into independent modules using dynamic dispatch:

The gui module defines interfaces for all the components.
It has no external dependencies.
The html_gui module provides HTML implementation of the base GUI.
Depends on gui.
The windows_gui module provides Windows implementation of the base GUI.
Depends on gui.
The app is a client application that can use several implementations of the GUI framework, depending on the current environment or configuration. However, most of the app code doesn't depend on specific types of GUI elements. All client code works with GUI elements through abstract interfaces defined by the gui module.

引用：https://refactoringguru.cn/design-patterns/factory-method/rust/example#example-0