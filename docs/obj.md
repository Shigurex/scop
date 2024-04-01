# .obj format
.obj format is a format to store geometry in Wavefront's Advanced Visualizer written in ASCII format. In this project, following things will be implemented.

|data|information|format|detail|
|-|-|-|-|
|#|comment|# <comment\>|a phrase written after # will be ignored|
|mtllib|material file|mtllib <file name\>|setting a file with the material definition|
|o|object name|o <object name\>|give name to an object|
|v|geometric vertices|v <x\> <y\> <z\> <(w)\>|define position of the vertice|
|usemtl|material|usemtl <color or material\>|define a color or material of the object|
|s|smooth shading|s <on or off\>|turn on smooth shading (flat shading is used by default)|
|f|face|f <v1\> <v2\> <v3\> ...|define vertices that compose a face|

> for more detail: https://paulbourke.net/dataformats/obj/ 
> https://learnwebgl.brown37.net/modelers/obj_data_format.html