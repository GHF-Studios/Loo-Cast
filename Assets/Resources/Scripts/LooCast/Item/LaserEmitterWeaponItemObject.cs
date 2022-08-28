using System;
using UnityEngine;

namespace LooCast.Item
{
    using Data;
    using LooCast.Util;
    using LooCast.Attribute.Stat;

    public class LaserEmitterWeaponItemObject : UniqueItemObject
    {
        [SerializeField] protected LaserEmitterWeaponItemData data;

        public override Item Item
        {
            set
            {
                base.Item = value;
                LaserEmitterWeaponItem = (LaserEmitterWeaponItem)value;
                Refresh();
            }
        }
        public LaserEmitterWeaponItem LaserEmitterWeaponItem { get; protected set; }

        [SerializeField] private Stats stats;

        private void Awake()
        {
            //First we initialize independent members
            LaserEmitterWeaponItem = new LaserEmitterWeaponItem(data, this, stats);
            LaserEmitterWeaponItem.OnDrop.Invoke();
            SpriteRenderer = GetComponent<SpriteRenderer>();

            //Then we Initialize the ItemObject, as this sets the item and thus triggers Refresh, which needs to happen after Independent members have been initialized
            Initialize(LaserEmitterWeaponItem);
        }

        public void Refresh()
        {
            
        }
    }
}