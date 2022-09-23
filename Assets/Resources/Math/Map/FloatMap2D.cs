using System;
using UnityEngine;

namespace LooCast.Math.Map
{
    [Serializable]
    public struct FloatMap2D
    {
        public float[,] Array2D
        {
            get
            {
                float[,] array2D = new float[arrayWidth, arrayHeight];
                for (int y = 0; y < arrayHeight; y++)
                {
                    for (int x = 0; x < arrayWidth; x++)
                    {
                        array2D[x, y] = array1D[y * arrayWidth + x];
                    }
                }
                return array2D;
            }
        }

        [SerializeField] private float[] array1D;
        [SerializeField] private int arrayWidth;
        [SerializeField] private int arrayHeight;

        public FloatMap2D(float[,] array2D)
        {
            array1D = new float[array2D.Length];
            arrayWidth = array2D.GetLength(0);
            arrayHeight = array2D.GetLength(1);

            for (int y = 0; y < arrayHeight; y++)
            {
                for (int x = 0; x < arrayWidth; x++)
                {
                    array1D[y * arrayWidth + x] = array2D[x, y];
                }
            }
        }
    }
}