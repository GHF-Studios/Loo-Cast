using UnityEngine;

namespace LooCast.Mission.Data
{
    using LooCast.Data;
    using LooCast.Currency;

    public class ConquerStationMissionData : MissionData
    {
        public Credits Credits;
        public IntDataReference CreditsReward;

        public override Mission CreateMission(MissionProvider missionProvider)
        {
            return new ConquerStationMission(this, missionProvider);
        }
    }
}