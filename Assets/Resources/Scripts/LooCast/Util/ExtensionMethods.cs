using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Util
{
    using LooCast.Item;
    using LooCast.System;
    using LooCast.System.Identification;

    // TODO: Move to System namespace
    public static class ExtensionMethods
    {
        public static float Map(this float value, float fromMin, float fromMax, float toMin, float toMax)
        {
            return (value - fromMin) / (fromMax - fromMin) * (toMax - toMin) + toMin;
        }

        public static float Map(this int value, int fromMin, int fromMax, int toMin, int toMax)
        {
            return (value - fromMin) / (fromMax - fromMin) * (toMax - toMin) + toMin;
        }

        public static Item[] GetItems(this Dictionary<int, ItemContainerSlot> itemSlots)
        {
            Item[] items = new Item[itemSlots.Count];
            for (int i = 0; i < itemSlots.Count; i++)
            {
                items[i] = itemSlots[i].ItemContent;
            }
            return items;
        }

        public static T[] Cast<T>(this Item[] items) where T : Item
        {
            T[] castItems = new T[items.Length];
            for (int i = 0; i < items.Length; i++)
            {
                if (items[i] is not T)
                {
                    throw new InvalidCastException($"Item at index {i} is not of the Generic Type, that was provided, and could thus not be cast!");
                }
                castItems[i] = (T)items[i];
            }
            return castItems;
        }

        public static void Shuffle<T>(this List<T> list)
        {
            int n = list.Count;
            while (n > 1)
            {
                n--;
                int k = UnityEngine.Random.Range(0, n + 1);
                T value = list[k];
                list[k] = list[n];
                list[n] = value;
            }
        }

        public static T[] GetValues<T>(this Variable.Variable<T>[] valueVariables)
        {
            T[] evaluatedValues = new T[valueVariables.Length];
            for (int i = 0; i < valueVariables.Length; i++)
            {
                evaluatedValues[i] = valueVariables[i].Value;
            }
            return evaluatedValues;
        }

        public static Vector2 ToVector2(this Vector2Int vector2Int)
        {
            return new Vector2(vector2Int.x, vector2Int.y);
        }

        public static Vector2Int FloorToVector2Int(this Vector2 vector2)
        {
            return new Vector2Int(Mathf.FloorToInt(vector2.x), Mathf.FloorToInt(vector2.y));
        }

        public static Vector2Int CeilToVector2Int(this Vector2 vector2)
        {
            return new Vector2Int(Mathf.CeilToInt(vector2.x), Mathf.CeilToInt(vector2.y));
        }

        public static Vector2Int RoundToVector2Int(this Vector2 vector2)
        {
            return new Vector2Int(Mathf.RoundToInt(vector2.x), Mathf.RoundToInt(vector2.y));
        }

        public static double Microseconds(this TimeSpan timeSpan)
        {
            return timeSpan.Ticks / 10;
        }

        public static double Nanoseconds(this TimeSpan timeSpan)
        {
            return timeSpan.Ticks * 100;
        }

        public static double Pictoseconds(this TimeSpan timeSpan)
        {
            return timeSpan.Ticks * 100000;
        }

        public static double Femtoseconds(this TimeSpan timeSpan)
        {
            return timeSpan.Ticks * 100000000;
        }
    }
}
