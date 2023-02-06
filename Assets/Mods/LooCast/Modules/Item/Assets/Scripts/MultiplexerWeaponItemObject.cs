using System;
using UnityEngine;

namespace LooCast.Item
{
    using Data;
    using LooCast.Util;
    using LooCast.Attribute.Stat;

    public class MultiplexerWeaponItemObject : UniqueItemObject
    {
        [SerializeField] protected MultiplexerWeaponItemData data;

        public override Item Item
        {
            set
            {
                MultiplexerWeaponItem = (MultiplexerWeaponItem)value;
                if (MultiplexerWeaponItem == null)
                {
                    throw new ArgumentException("Invalid Item Type!");
                }
                base.Item = value;
                Refresh();
            }
        }
        public MultiplexerWeaponItem MultiplexerWeaponItem { get; protected set; }

        [SerializeField] private Stats stats;

        private void Awake()
        {
            SpriteRenderer = GetComponent<SpriteRenderer>();

            Initialize((MultiplexerWeaponItem)data.CreateItem());
        }

        public void Refresh()
        {
            
        }
    }
}