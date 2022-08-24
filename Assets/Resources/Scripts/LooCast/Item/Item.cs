using UnityEngine;

namespace LooCast.Item
{
    using Data;

    public abstract class Item
    {
        private static int IDCounter = 0;
        public int ID { get; private set; }
        public string Name { get; protected set; }
        public Sprite Sprite { get; protected set; }

        public Item(ItemData data)
        {
            ID = IDCounter;
            IDCounter++;
            Name = data.Name;
            Sprite = data.Sprite;
        }

        public override bool Equals(object obj)
        {
            Item item = (Item)obj;
            if (item != null && item.Name == Name)
            {
                return true;
            }
            return false;
        }

        public override int GetHashCode()
        {
            return ID;
        }
    }
}