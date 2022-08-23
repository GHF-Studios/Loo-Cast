using UnityEngine;

namespace LooCast.Item
{
    using Data;

    public abstract class Item
    {
        public abstract string Name { get; protected set; }

        public Item(string name)
        {
            Name = name;
        }
    }
}