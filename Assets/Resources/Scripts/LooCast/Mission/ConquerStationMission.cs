using System;
using System.Collections.Generic;
using UnityEngine.Events;

namespace LooCast.Mission
{
    using Data;

    public class ConquerStationMission : Mission
    {
        public ConquerStationMission(ConquerStationMissionData data, MissionProvider missionProvider) : base(data, missionProvider)
        {
            AddReward(new CreditsMissionReward(data.Credits, data.CreditsReward.Value));
        }
    }
}