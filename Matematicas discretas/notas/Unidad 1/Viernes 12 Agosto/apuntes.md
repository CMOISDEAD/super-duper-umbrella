<h1> 2022-08-12:  Tecnicas de conteo </h1>

[[toc]]

### Principio de la suma

##### _Definicion_

Si una tarea puede realizarse de $m$ formas, mientras que una segunda puede realizarse de $n$ formas,
y no es posible **realizar ambas tareas de forma simultanea**, entonces, para llevar a cabo cualquiera
de ellas pude utilizarse cualquiera de $m + n$ formas.

##### _Ejemplos_

**Ejemplo 1.1**

La biblioteca de la universidad tiene 40 libros de texto de sociologia y 50 de antropologia. Por la regla
de la suma, un estudiante de esta universidad puede elegirentre $40 + 50 = 90$ libros de texto para aprender
acerca de alguno de estos dos temas.

### Principio del producto

##### _Definicion_

Si un suceso $A$ ocurre de $m$ maneras, mientras que un suceso $B$ ocurre de $n$ maneras y ambas sucesos
ocurren de manera simultanea. para llevar acabo este procemiento puede utilizarse $mxn$maneras

- Si un suceso $A$ ocurre de $A$ maneras
- Si un suceso $B$ ocurre de $B$ maneras
- Si un suceso $C$ ocurre de $C$ maneras

Todos estos sucesos pueden ocurrir de manera simulatean de:

$$
  A.B.C...Z
$$

##### _Ejemplos_

**Ejemplo 1.1**

Supongamos que se toman tienen 5 libros de calculo t 10 libros de probabilidad.

de cuantas maneras se puede elegir uno de estos libros?

$$
  5 + 10 = 15
$$

de cuntas maneras se puede escoger 2 de estos libros?

$$
  15 . 14 =  210
$$

### Diagrama de arbol

#### _Definicion_

El diagrama de árbol es una representación gráfica de los posibles
resultados del experimento, el cual consta de una serie de pasos, donde cada
uno de estos tiene un número infinito de maneras de ser llevado a cabo.
Se utiliza en los problemas de conteo y probabilidad.

#### _Ejemplo_

**Ejemplo 1.1**

Se desea escoger dos estudiantes entre Pablo, Maria, Camila y Pedro para un concurso,
el primero sera el principal y el segundo sera el suplente.

de cuantas maneras se puede escoger los estudiantes?

- Suceso $A$: 5
- Suceso $A$: 4

$$
  5.4=20
$$

**Diagrama**

![ejemplo](http://2.bp.blogspot.com/-St-1XWc-nLI/T8K33YwJBYI/AAAAAAAAAJU/1eQcUJLBr-s/s1600/Imagen5.png)

<p align="center" style="font-style: italic">Ejemplo de diagrama de arbol.</p>

### Metodo de la caja

#### _Ejemplo_

Con los digitos primos se desea formar numeros de cinco cifras.

$$
A =\{ 2, 3, 5, 7 \}
$$

1. Cuantos numeros se pueden formar ?

$$4.4.4.4.4 = 1024$$

2. Cuantos de esos numeros son mayores de 50.000?

$$2.4.4.4.4 = 512$$

3. Cuantos de esos numeros son mentores de 20.000?
   $$0$$

### Permutacion

De un grupo de $n$ objetos deseo escoger $r$ de ellos en sin importar el orden y donde no se permita
repetir elementos. si cumplen estas caracteristicas podemos utilizar la formula de la repeticion.

$$
  P(n,r) = nPr = \frac{n!}{(n-r)!}
$$

#### _Ejemplos_

$n = 10$

$r = 3$

$$
  _{10}P_3 = \frac{10!}{(10-3)!}
  = \frac{10!}{7!}
  = \frac{7!.8.9.10}{7!}
$$

Podemos concluir que se pueden entregar las medallas de 720 maneras.

#### _Ejercicio_

_Ejercicio 1.1_

Una empresa realiza una fiesta y desea dar un televisor, una nevera y una grabadora,
entre sus 80 trabajadores.

- cuantas maneras se pueden repartir los electrodomesticos si solo se da un premio por
  persona?

$P=80$

**Metodo de la caja**

$$
  80.79.78 = 492.960
$$

**Metodo por permutacion**

$$
  _{80}P_3 = P(80, 3)
  = \frac{80!}{(80-3)!}
  = \frac{77!.78.79.80}{77!}
  = 78.79.80 = 492.960
$$

<strong style="color: red">Ejercicio 1.2</strong>

demuestra que para cualquier pareja de numeros donde $n,r \geq 0$ si $n+1$ es mayor que $r$
entoces la permutacion de:

$$
  P(n+1, r) = \left(\frac{n+1}{n+1-r}\right) . P(n, r)
$$

**Proceso**

$$
\begin{align*}
  P(n+1, r) &= \left(\frac{n+1}{n+1-r}\right) . P(n, r)\\
  = \frac{(n+1)!}{(n-1-r)!} &= \frac{n+1}{n+1-r} . \frac{n!}{(n-r)!} \\
  = P(n+1, r) &= \frac{n+1}{n+1-r} . \frac{n!}{(n-r)!}
\end{align*} 
$$

_Ejercicio 1.3_

Determine el valor de n si  $P(n, 3) = 3 P(n, 2)$.

**Proceso**

$$
\begin{align*}
  P(n, 3) &= 3.P(n, 2) \\
  \frac{n!}{(n-3)!} &= 3.\frac{n!}{(n-2)!} \\
  \frac{1}{(n-3)!} &= \frac{3}{(n-2)!} \\
  \frac{(n-2)!}{(n-3)!} &= 3 \\
  \frac{(n-3)!.(n-2)}{(n-3)!} &= 3 \\
  n.2 &= 3 \\
  n &= 5
\end{align*} 
$$

***
