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
            SpriteRenderer = GetComponent<SpriteRenderer>();

            Initialize(LaserEmitterWeaponItem);
        }

        public void Refresh()
        {
            
        }
    }
}