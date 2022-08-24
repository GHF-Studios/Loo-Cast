using UnityEngine;

namespace LooCast.Item
{
    using Data;

    public abstract class Item
    {
        public string Name { get; protected set; }
        public Sprite Sprite { get; protected set; }

        public Item(ItemData data)
        {
            Name = data.Name;
            Sprite = data.Sprite;
        }
    }
}