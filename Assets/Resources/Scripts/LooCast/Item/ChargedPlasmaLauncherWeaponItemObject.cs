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
                base.Item = value;
                ChargedPlasmaLauncherWeaponItem = (ChargedPlasmaLauncherWeaponItem)value;
                Refresh();
            }
        }
        public ChargedPlasmaLauncherWeaponItem ChargedPlasmaLauncherWeaponItem { get; protected set; }

        [SerializeField] private Stats stats;

        private void Awake()
        {
            //First we initialize independent members
            ChargedPlasmaLauncherWeaponItem = new ChargedPlasmaLauncherWeaponItem(data, this, stats);
            ChargedPlasmaLauncherWeaponItem.OnSpawn.Invoke();
            SpriteRenderer = GetComponent<SpriteRenderer>();

            //Then we Initialize the ItemObject, as this sets the item and thus triggers Refresh, which needs to happen after Independent members have been initialized
            Initialize(ChargedPlasmaLauncherWeaponItem);
        }

        public void Refresh()
        {
            
        }
    }
}