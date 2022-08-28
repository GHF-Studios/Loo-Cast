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

        public static Item[] GetItems<T>(this Dictionary<int, ItemContainerSlot<T>> itemSlots) where T : Item
        {
            Item[] items = new Item[itemSlots.Count];
            for (int i = 0; i < itemSlots.Count; i++)
            {
                items[i] = itemSlots[i].ItemContent;
            }
            return items;
        }
    }
}
