using UnityEngine;

namespace LooCast.Util
{
    public static class TextureUtil
    {
        public static Texture2D TextureFromColorMap(Color[] colorMap, int width, int height)
        {
            Texture2D texture = new Texture2D(width, height);
            texture.filterMode = FilterMode.Point;
            texture.wrapMode = TextureWrapMode.Clamp;
            texture.SetPixels(colorMap);
            texture.Apply();
            return texture;
        }

        public static Texture2D TextureFromHeightMap(float?[,] heightMap)
        {
            int width = heightMap.GetLength(0);
            int height = heightMap.GetLength(1);

            Color[] colorMap = new Color[width * height];
            for (int y = 0; y < height; y++)
            {
                for (int x = 0; x < width; x++)
                {
                    if (heightMap[x, y] == null)
                    {
                        colorMap[y * width + x] = Color.black;
                    }
                    else
                    {
                        colorMap[y * width + x] = Color.Lerp(Color.black, Color.white, (float)heightMap[x, y]);
                    }
                }
            }

            return TextureFromColorMap(colorMap, width, height);
        }
    } 
}
