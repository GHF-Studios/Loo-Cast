using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.UI.Button
{
    using LooCast.Mission;

    public class MissionButton : Button
    {
        public Mission Mission { get; private set; }

        public void Initialize(Mission mission)
        {
            Mission = mission;
        }

        public override void OnClick()
        {
            
        }
    }
}
