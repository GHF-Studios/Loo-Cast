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
                base.Item = value;
                MultiplexerWeaponItem = (MultiplexerWeaponItem)value;
                Refresh();
            }
        }
        public MultiplexerWeaponItem MultiplexerWeaponItem { get; protected set; }

        [SerializeField] private Stats stats;

        private void Awake()
        {
            //First we initialize independent members
            MultiplexerWeaponItem = new MultiplexerWeaponItem(data, this, stats);
            MultiplexerWeaponItem.OnDrop.Invoke();
            SpriteRenderer = GetComponent<SpriteRenderer>();

            //Then we Initialize the ItemObject, as this sets the item and thus triggers Refresh, which needs to happen after Independent members have been initialized
            Initialize(MultiplexerWeaponItem);
        }

        public void Refresh()
        {
            
        }
    }
}