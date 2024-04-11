# Parsing Text File
## Object File Format
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
> https://ja.wikipedia.org/wiki/Wavefront_.obj%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB

## Material File Format
.mtl format is a format to store information about shading material developed by same 

|data|information|format|detail|
|-|-|-|-|
|newmtl|material name|newmtl <material name\>|define a material name|
|Ka|ambient color|Ka <r\> <g\> <b\>|define ambient color|
|Kd|diffuse color|Kd <r\> <g\> <b\>|define diffuse color|
|Ks|specular color|Ks <r\> <g\> <b\>|define specular color|
|Ni|optical density|Ni <optical density\>|define optical density|
|d|transparency|d <transparency\>|define transparency|
|illum|illumination model|illum <illumination model\>|define illumination model|
<!-- |Tr|transparency|Tr <transparency\>|define transparency| -->
<!-- |Ns|specular exponent|Ns <shininess\>|define specular exponent| -->


