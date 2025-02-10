## Création des objets

### [Cube](./src/geometry/objects/cube.rs)

Le cube est défini par son point central et sa taille (longueur d'arête). Un point se trouve sur le cube si sa distance par rapport au centre le long de chaque axe est inférieure ou égale à la moitié de la taille. Cela peut s'exprimer par un ensemble d'inégalités pour chaque axe :

$$
|x - C_x| \leq \frac{s}{2} \quad \text{et} \quad |y - C_y| \leq \frac{s}{2} \quad \text{et} \quad |z - C_z| \leq \frac{s}{2}
$$

Où :
- $(C_x, C_y, C_z)$ est le centre du cube
- $s$ est la taille (longueur d'arête) du cube

Pour trouver les intersections avec un rayon $P(t) = A + t\vec{b}$, nous utilisons la "méthode des dalles". Celle-ci implique :

1. De considérer le cube comme l'intersection de trois paires de plans parallèles (dalles)
2. De trouver les points d'intersection avec chaque dalle
3. De prendre le plus grand point d'entrée ($t_{near}$) et le plus petit point de sortie ($t_{far}$)

Pour chaque axe, les temps d'intersection $t_1$ et $t_2$ avec la dalle correspondante sont :

$$
t_1 = \frac{(C_i - \frac{s}{2}) - A_i}{b_i} \quad \text{et} \quad t_2 = \frac{(C_i + \frac{s}{2}) - A_i}{b_i}
$$

Où $i$ représente chaque axe $(x, y, z)$.

L'intersection finale se produit si et seulement si :
$$
\max(t_{near}) \leq \min(t_{far})
$$

### [Cylindre](./src/geometry/objects/cylinder.rs)

Un cylindre est défini par son point central, son rayon, sa hauteur et son vecteur d'orientation. L'intersection avec un cylindre implique de vérifier à la fois sa surface courbe et ses faces. Un point se trouve sur le cylindre s'il satisfait l'une de ces conditions :

1. Pour la surface courbe : La distance du point à l'axe du cylindre est égale au rayon, et la hauteur du point le long du vecteur d'orientation est comprise entre 0 et la hauteur du cylindre.
2. Pour les faces : Le point se trouve dans le rayon de l'une des faces circulaires (supérieure ou inférieure).

Pour la surface courbe, étant donné un point $P$, nous pouvons exprimer ces conditions mathématiquement :

$$
\text{Soit } \vec{v} = \vec{P} - \vec{C} \text{ (vecteur du centre au point)}
$$
$$
\text{Soit } h = \vec{v} \cdot \hat{d} \text{ (hauteur le long du vecteur d'orientation } \hat{d}\text{)}
$$
$$
\text{Soit } \vec{r} = \vec{v} - h\hat{d} \text{ (vecteur rayon)}
$$

Alors le point se trouve sur la surface courbe si :
$$
\vec{r} \cdot \vec{r} = R^2 \quad \text{et} \quad 0 \leq h \leq H
$$

Pour un rayon $P(t) = A + t\vec{b}$, la substitution et la résolution conduisent à une équation quadratique :

$$
((\vec{b} \cdot \vec{b}) - (\vec{b} \cdot \hat{d})^2)t^2 + 2(\vec{b} \cdot \vec{w} - (\vec{b} \cdot \hat{d})(\vec{w} \cdot \hat{d}))t + (\vec{w} \cdot \vec{w} - (\vec{w} \cdot \hat{d})^2 - R^2) = 0
$$

Où $\vec{w} = A - C$

Pour les faces, nous vérifions l'intersection avec deux plans aux hauteurs 0 et H, puis nous vérifions si le point d'intersection se trouve dans le rayon :

$$
t = \frac{(C + h\hat{d} - A) \cdot \hat{d}}{\vec{b} \cdot \hat{d}}
$$

Où $h$ est soit 0 soit H pour les faces inférieure et supérieure respectivement.





## Création des objets géométriques des exemples plus simple et comprehensive

### [Cube](./src/geometry/objects/cube.rs)

Le cube est défini par trois éléments simples :
- Son centre (point au milieu du cube)
- Sa taille (longueur d'un côté)
- Son matériau (pour le rendu)

#### Comment ça marche ?

Imaginez un cube de taille 2 centré en (0,0,0) :
- Sa face gauche sera à x = -1
- Sa face droite sera à x = 1
- Sa face inférieure sera à y = -1
- Sa face supérieure sera à y = 1
- Sa face avant sera à z = -1
- Sa face arrière sera à z = 1

Pour détecter une collision avec un rayon, on vérifie :
1. Quand le rayon entre dans le cube (tₑₙₜᵣéₑ)
2. Quand le rayon sort du cube (tₛₒᵣₜᵢₑ)

Exemple simple :
```
Si un rayon part du point (0,0,-5) dans la direction (0,0,1)
→ Il touchera la face avant du cube quand z = -1
→ Donc tₑₙₜᵣéₑ = 4 (car il faut parcourir 4 unités pour aller de -5 à -1)
→ Il sortira quand z = 1
→ Donc tₛₒᵣₜᵢₑ = 6 (car il faut parcourir 6 unités pour aller de -5 à 1)
```

### [Cylindre](./src/geometry/objects/cylinder.rs)

Le cylindre est défini par :
- Son centre (point au milieu de la base)
- Son rayon (distance du centre au bord)
- Sa hauteur (distance de la base au sommet)
- Son orientation (direction dans laquelle il pointe)
- Son matériau (pour le rendu)

#### Comment vérifier si un point est sur le cylindre ?

1. Pour la surface courbe :
   ```
   Exemple : Cylindre vertical de rayon 2 et hauteur 4
   - Centre : (0,0,0)
   - Point P : (2,0,1)
   
   → Distance horizontale au centre = 2 (égale au rayon ✓)
   → Hauteur = 1 (entre 0 et 4 ✓)
   → Le point est sur la surface !
   ```

2. Pour les faces (haut/bas) :
   ```
   Exemple : Même cylindre
   - Point P : (1,1,4)
   
   → Distance au centre = √(1² + 1²) = √2 < 2 (dans le rayon ✓)
   → Hauteur = 4 (sur la face supérieure ✓)
   → Le point est sur la face supérieure !
   ```

#### Calcul d'intersection avec un rayon

Pour la surface courbe, on cherche quand :
1. La distance à l'axe = rayon
2. La hauteur est entre 0 et la hauteur du cylindre

Exemple simple :
```
Cylindre vertical : rayon = 1, hauteur = 3
Rayon partant de (-2,0,1) vers (1,0,0)

1. Le rayon touchera la surface à x = -1 (première intersection)
2. Puis à x = 1 (deuxième intersection)
3. Comme y = 0 et z = 1 sont constants et que 1 est entre 0 et 3,
   ces deux points sont des intersections valides !
```

Pour les faces, on vérifie :
1. Quand le rayon atteint la hauteur de chaque face
2. Si le point d'intersection est dans le cercle de la face

Exemple :
```
Même cylindre
Rayon partant de (0.5,0.5,4) vers (0,0,-1)

→ Touchera la face supérieure en z = 3
→ Point d'intersection : (0.5,0.5,3)
→ Distance au centre = √(0.5² + 0.5²) ≈ 0.7 < 1
→ C'est une intersection valide !
```

### Notes importantes

- Pour les deux objets, on garde toujours l'intersection la plus proche du point de départ du rayon
- Si le rayon part de l'intérieur de l'objet, on utilise la première sortie comme point d'intersection
- Les calculs sont optimisés dans le code pour être plus rapides, mais suivent cette même logique