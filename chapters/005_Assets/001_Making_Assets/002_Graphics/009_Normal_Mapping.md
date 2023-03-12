### Normal Mapping

If you want your 2D game to look amazing, you can't escape shaders. One of the ways to use shaders is calculating light, but since we're in a 2D environment, we don't have "normal vectors" to use to calculate how the light interacts with our 2D objects.

Enter normal maps: these are sprites that "map" their color channels to the direction of the "normal vector": this means that communicate direction with color. Awesome, isn't it?

::: trivia :::
If you're curious, normal maps usually have this color to direction mapping:

- x (which can go from -1 to +1) is mapped to the red channel (which can go from 0 to 255)
- y (which can go from -1 to +1) is mapped to the green channel (which can go from 0 to 255)
- z (which can go from 0 to -1) is mapped to the blue channel (which can go from 128 to 255)
::::::::::::::

This is an example of a texture, along with a possible normal map:

![A texture (on theleft), with a possible normal map (on the right)](./images/resources/normal_mapping_1.jpg){width=50%}

There are many ways to get a normal map: you can try to get a program to generate one for you, make the object in a 3D modeler and extract the normal map from there, or just draw it by hand.

If you choose the last option some tools, like Aseprite, have a "normal mapping" mode that shows you this special color picker:

![Aseprite's normal mapping color picker (both in its normal and discrete versions)](./images/resources/normal_mapping_2.png){width=40%}

Just imagine this color picker like a "3D sphere" and pick the color of the face of the "3d surface" you're trying to draw.

::: pitfall :::
Be careful with your shaders, some may expect one or more of your channels to be "flipped".
:::::::::::::::

#### A simple example

Let's take a simple box, with no shading, like the one below:

![A box that will be used to show how normal maps influence light](./images/resources/Box_No_Light.png){width=25%}

Now we'll shine a light on the box, without any normal map: this will happen twice:

In the first example (left) the light will be a round gradient that will come from the top right corner of the image, while in the second example the light will be a bit stronger and coming from the top left corner. This is the result.

![How the lack of normal mapping makes lighting look artificial](./images/resources/No_Normal_Map.png){width=35%}

Now we'll draw the simplest normal map possible: just filling the 3 faces of the box that we can see with the (kind of) corresponding colors from our "3d sphere" (the normal mapping color picker), we can see the result is very different

![How normal mapping changes lighting](./images/resources/Normal_Map_Rough.png){width=60%}

Now let's make something a bit more detailed, by highlighting the faces of the cross-braces on the sides of the box, the way they're lit it's again different:

![A more detailed normal map results in better lighting](./images/resources/Normal_Map_Detailed.png){width=60%}

You can get as detailed as you want, but remember that it may have some performance impact if you go overboard with many sprites.
