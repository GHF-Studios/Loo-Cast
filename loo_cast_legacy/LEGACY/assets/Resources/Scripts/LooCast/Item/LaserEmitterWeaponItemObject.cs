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
                LaserEmitterWeaponItem = (LaserEmitterWeaponItem)value;
                if (LaserEmitterWeaponItem == null)
                {
                    throw new ArgumentException("Invalid Item Type!");
                }
                base.Item = value;
                Refresh();
            }
        }
        public LaserEmitterWeaponItem LaserEmitterWeaponItem { get; protected set; }

        [SerializeField] private Stats stats;

        private void Awake()
        {
            SpriteRenderer = GetComponent<SpriteRenderer>();

            Initialize((LaserEmitterWeaponItem)data.CreateItem());
        }

        public void Refresh()
        {
            
        }
    }
}