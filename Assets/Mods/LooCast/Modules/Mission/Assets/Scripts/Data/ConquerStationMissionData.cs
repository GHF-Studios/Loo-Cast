using UnityEngine;

namespace LooCast.Mission.Data
{
    using LooCast.Data;
    using LooCast.Currency;
    using LooCast.Item.Data;
    using LooCast.Inventory.Data.Runtime;

    [CreateAssetMenu(fileName = "ConquerStationMissionData", menuName = "Data/Mission/ConquerStationMissionData", order = 0)]
    public class ConquerStationMissionData : MissionData
    {
        public Credits Credits;
        public PlayerInventoryRuntimeData PlayerInventoryRuntimeData;
        public IntDataReference CreditsReward;
        public IntDataReference ReputationReward;
        public ItemData ItemReward;
        public IntDataReference RequiredEnemyKillCount;

        public override Mission CreateMission(MissionProvider missionProvider)
        {
            return new ConquerStationMission(this, missionProvider);
        }
    }
}