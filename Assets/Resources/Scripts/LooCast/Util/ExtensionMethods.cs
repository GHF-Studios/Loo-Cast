using System;
using System.Collections.Generic;

namespace LooCast.Util
{
    using LooCast.Item;

    public static class ExtensionMethods
    {
        public static float Map(this float value, float fromMin, float fromMax, float toMin, float toMax)
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
    }
}
