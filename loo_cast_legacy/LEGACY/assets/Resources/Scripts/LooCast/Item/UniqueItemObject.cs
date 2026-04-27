using System;
using UnityEngine;

namespace LooCast.Item
{
    using Data;

    public abstract class UniqueItemObject : ItemObject
    {
        public UniqueItem UniqueItem { get; protected set; }
        public override Item Item
        {
            set
            {
                UniqueItem = (UniqueItem)value;
                if (UniqueItem == null)
                {
                    throw new ArgumentException("Invalid Item Type!");
                }
                base.Item = value;
            }
        }

        protected void Initialize(UniqueItem uniqueItem)
        {
            base.Initialize(uniqueItem);
            UniqueItem = uniqueItem;
        }
    }
}