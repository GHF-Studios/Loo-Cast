using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

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
    }
}
