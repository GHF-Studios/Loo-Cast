using UnityEngine;

namespace LooCast.Item.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "MultiplexerWeaponItemData", menuName = "Data/Item/Weapon/MultiplexerWeaponItemData", order = 0)]
    public class MultiplexerWeaponItemData : WeaponItemData
    {
        public IntDataReference BaseMaxTargets;
        public IntDataReference BaseMaxFragments;
        public IntDataReference BaseFragmentArmorPenetration;
        public BoolDataReference IsTargetSeeking;
        public GameObject FragmentPrefab;

        public override Item CreateItem()
        {
            return new MultiplexerWeaponItem(this, Stats, AutoFire.Value);
        }
    }
}