#version 410 core
in vec4 TEPosition;
in vec3 TENormal;
in vec2 TexCoord;
in mat3 TBN;
out vec4 FragColor;

uniform vec3 ourColor;
uniform vec3 lightPos;
uniform float ks;
uniform float kd;
uniform uint m;
uniform vec3 lightColor;
uniform vec3 objectColor;
uniform vec3 cameraPos;
uniform sampler2D ourTexture;
uniform sampler2D normalMap;
uniform bool isTextureSet;
uniform bool isNormalMapSet;
uniform bool main_light;
uniform bool reflectors;

void main() {
   vec3 normal = vec3(0,0,1);
   if(isNormalMapSet) normal = normalize(texture(normalMap, TexCoord).rgb * 2.0 -1.0);

    normal = -normalize(TBN * normal);

    vec3 lightVector = normalize(lightPos - TEPosition.xyz);
    vec3 V = normalize(cameraPos - TEPosition.xyz);

    float cos_n_l = dot(normal,lightVector);
    float cos_R_l_m = dot(V,reflect(normal,lightVector));

    cos_n_l = max(0.0,min(1.0,cos_n_l));
    cos_R_l_m = pow(max(0.0,min(1.0,cos_R_l_m)),m);

    float h = 1;//lightPos.z;

    vec3 red_pos = vec3(-0.75,-0.75,h);
    vec3 green_pos = vec3(0.75,-0.75,h);
    vec3 blue_pos = vec3(0,0.75,h);
    vec3 pos = TEPosition.xyz;
    vec3 target = vec3(0.0,0.0,0.0);

    vec3 red_color = vec3(255,0,0);
    vec3 green_color = vec3(0,255,0);
    vec3 blue_color = vec3(0,0,255);


   vec3 l_red =  -normalize(pos - red_pos);
 vec3 l_green = -normalize(pos - green_pos);
 vec3 l_blue = -normalize(pos - blue_pos);

 vec3   d_red = -normalize(target - red_pos);
 vec3  d_green = -normalize(target - green_pos);
 vec3   d_blue = -normalize(target - blue_pos);

 int k = 200;

 vec3  Ir = red_color * pow(max(0,min(1.0,dot(l_red,d_red))),k);
 vec3   Ig = green_color * pow(max(0,min(1.0,dot(l_green,d_green))),k);
 vec3  Ib = blue_color * pow(max(0,min(1.0,dot(l_blue,d_blue))),k);

 float cos_n_l_r = max(0.0,min(1.0,dot(normal,l_red)));
 float cos_n_l_g = max(0.0,min(1.0,dot(normal,l_green)));
 float cos_n_l_b = max(0.0,min(1.0,dot(normal,l_blue)));

 float cos_R_l_m_r = pow(max(0.0,min(1.0,dot(V,reflect(normal,l_red)))),m);
 float cos_R_l_m_g = pow(max(0.0,min(1.0,dot(V,reflect(normal,l_green)))),m);
 float cos_R_l_m_b = pow(max(0.0,min(1.0,dot(V,reflect(normal,l_blue)))),m);


 vec3 rgb = kd * lightColor * objectColor * cos_n_l + ks * lightColor * objectColor * cos_R_l_m;
 vec3 red_reflector = kd *objectColor  * Ir * cos_n_l_r + ks * Ir * objectColor * cos_R_l_m_r;
 vec3 blue_reflector = kd *objectColor  * Ib * cos_n_l_b + ks * Ib * objectColor * cos_R_l_m_b;
 vec3 green_reflector = kd *objectColor  * Ig * cos_n_l_g + ks * Ig * objectColor * cos_R_l_m_g;

 vec3 startColor = vec3(0.0,0.0,0.0);

 if(reflectors) startColor += red_reflector + blue_reflector + green_reflector;
 if(main_light) startColor += rgb;

 vec4 normal_text = texture(normalMap,TexCoord);
 vec4 tex = texture(ourTexture,TexCoord);
 FragColor = vec4(startColor,1.0);


 if(isTextureSet) FragColor = tex * FragColor;

}