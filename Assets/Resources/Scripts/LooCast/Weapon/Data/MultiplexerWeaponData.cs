using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Weapon.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "MultiplexerWeaponData", menuName = "Data/Weapon/MultiplexerWeaponData", order = 0)]
    public sealed class MultiplexerWeaponData : WeaponData
    {
        public IntDataReference BaseMaxTargets;
        public IntDataReference BaseMaxFragments;
        public IntDataReference BaseFragmentArmorPenetration;
        public BoolDataReference IsTargetSeeking;
        public GameObject FragmentPrefab;
    } 
}
