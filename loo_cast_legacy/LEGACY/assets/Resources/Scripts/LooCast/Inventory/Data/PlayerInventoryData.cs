using UnityEngine;

namespace LooCast.Inventory.Data
{
    using LooCast.Item.Data;
    using LooCast.Data;

    [CreateAssetMenu(fileName = "PlayerInventoryData", menuName = "Data/Inventory/PlayerInventoryData", order = 0)]
    public class PlayerInventoryData : ScriptableObject
    {
        public IntDataReference SlotCount;
        public ChargedPlasmaLauncherWeaponItemData ChargedPlasmaLauncherWeaponItemData;
        public FreezeRayWeaponItemData FreezeRayWeaponItemData;
        public LaserEmitterWeaponItemData LaserEmitterWeaponItemData;
        public MultiplexerWeaponItemData MultiplexerWeaponItemData;
    } 
}
