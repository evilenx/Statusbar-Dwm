# StatusBar 
![statusbar](/screenshot/picture.jpg) 

StatusBar que imprime la hora en formato de hora, minutos, segundos y nanosegundos. 



## Instrucciones para Usar el Makefile

Este proyecto utiliza un archivo Makefile para compilar y gestionar el proceso de construcción. Siga estos pasos para compilar y utilizar el proyecto:

1. Asegúrese de que Rust y Cargo estén instalados en su sistema. Puede instalarlos desde [rustup](https://rustup.rs/).

2. Clone este repositorio de GitHub en su máquina:

   ```bash
   git clone https://github.com/evilenx/Statusbar.Dwm.git
   cd Statusbar-Dwm
   ```
Para compilar la Statusbar:

   ```bash
   make build
   ```
   
   Para instalar el ejecutable en su sistema, utilce el comando (puede requerir permisos de administrador):

   ```bash
   make install 
   ```

   Para desinstalar el ejecutable en su sistema, utilizar el comando: 
   ```bash
   make uninstall 
   ```

   Limpiar los archivos generados por la compilación: 
   ```bash
   make clean  
   ```
## Statusbar para dwm

Este proyecto proporciona un statusbar personalizado diseñado para funcionar con el gestor de ventanas [dwm](https://dwm.suckless.org/). La statusbar muestra información relevante, como la hora actual, directamente en tu barra de estado de dwm.

Con estos pasos, podrá compilar, instalar y desinstalar el proyecto fácilmente utilizando el Makefile proporcionado.

### Requisitos

- [dwm](https://dwm.suckless.org/): Asegúrese de tener dwm instalado en su sistema antes de utilizar este statusbar.

