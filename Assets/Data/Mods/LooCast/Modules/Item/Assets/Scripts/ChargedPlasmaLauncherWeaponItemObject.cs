using System;
using UnityEngine;

namespace LooCast.Item
{
    using Data;
    using LooCast.Util;
    using LooCast.Attribute.Stat;

    public class ChargedPlasmaLauncherWeaponItemObject : UniqueItemObject
    {
        [SerializeField] protected ChargedPlasmaLauncherWeaponItemData data;

        public override Item Item
        {
            set
            {
                ChargedPlasmaLauncherWeaponItem = (ChargedPlasmaLauncherWeaponItem)value;
                if (ChargedPlasmaLauncherWeaponItem == null)
                {
                    throw new ArgumentException("Invalid Item Type!");
                }
                base.Item = value;
                Refresh();
            }
        }
        public ChargedPlasmaLauncherWeaponItem ChargedPlasmaLauncherWeaponItem { get; protected set; }

        [SerializeField] private Stats stats;

        private void Awake()
        {
            SpriteRenderer = GetComponent<SpriteRenderer>();

            Initialize((ChargedPlasmaLauncherWeaponItem)data.CreateItem());
        }

        public void Refresh()
        {
            
        }
    }
}