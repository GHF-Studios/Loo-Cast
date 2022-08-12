using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Bar
{
    using LooCast.Health.Data.Runtime;

    public class HealthBar : Bar
    {
        public PlayerHealthRuntimeData PlayerHealthRuntimeData;

        private void Start()
        {
            PlayerHealthRuntimeData.MaxHealth.OnValueChanged.AddListener(() => { Refresh(); });
            PlayerHealthRuntimeData.Health.OnValueChanged.AddListener(() => { Refresh(); });
        }

        public override void Refresh()
        {
            Slider.minValue = 0.0f;
            Slider.maxValue = PlayerHealthRuntimeData.MaxHealth.Value;
            Slider.value = PlayerHealthRuntimeData.Health.Value;
        }
    }
}
