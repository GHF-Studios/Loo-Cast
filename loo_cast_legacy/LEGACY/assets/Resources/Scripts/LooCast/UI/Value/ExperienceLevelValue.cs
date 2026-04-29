using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Value
{
    using LooCast.Experience.Data.Runtime;

    public class ExperienceLevelValue : Value
    {
        public PlayerExperienceRuntimeData PlayerExperienceRuntimeData;

        public override void Refresh()
        {
            Text.text = $"Lvl {PlayerExperienceRuntimeData.CurrentLevel.Value}";
        }
    }
}
