using System.Collections.Generic;
using UnityEngine.Events;

namespace LooCast.Mission
{
    public class ConquerStationMission : Mission
    {
        public ConquerStationMission(ConquerStationMissionData data) : base(data)
        {
            AddReward(new CreditsMissionReward(data.Credits, data.CreditsReward));
        }
    }
}