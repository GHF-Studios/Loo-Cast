using System;
using UnityEngine;

namespace LooCast.System.Collections.Generic
{
    [Serializable]
    // Note: Can only be correctly serialized, if T is Serializable, too
    public struct Map2D<T>
    {
        public T[,] Array2D
        {
            get
            {
                T[,] array2D = new T[arrayWidth, arrayHeight];
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

        [SerializeField] private T[] array1D;
        [SerializeField] private int arrayWidth;
        [SerializeField] private int arrayHeight;

        public Map2D(T[,] array2D)
        {
            array1D = new T[array2D.Length];
            arrayWidth = array2D.GetLength(0);
            arrayHeight = array2D.GetLength(1);

            for (int y = 0; y < arrayHeight; y++)
            {
                for (int x = 0; x < arrayWidth; x++)
                {
                    array1D[Get1DIndex(x, y)] = array2D[x, y];
                }
            }
        }

        public Map2D(int width, int height)
        {
            array1D = new T[width * height];
            arrayWidth = width;
            arrayHeight = height;
        }

        public T GetValue(int xIndex, int yIndex)
        {
            return array1D[Get1DIndex(xIndex, yIndex)];
        }

        public void SetValue(int xIndex, int yIndex, T value)
        {
            array1D[Get1DIndex(xIndex, yIndex)] = value;
        }

        private int Get1DIndex(int xIndex, int yIndex)
        {
            return yIndex * arrayWidth + xIndex;
        }
    }
}