using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Bar
{
    using LooCast.Movement.Data.Runtime;

    public class EnergyBar : Bar
    {
        public PlayerMovementRuntimeData PlayerMovementRuntimeData;

        [SerializeField] private Image sliderImage;

        public override void Refresh()
        {
            Slider.minValue = 0.0f;
            Slider.maxValue = PlayerMovementRuntimeData.MaxEnergy.Value;
            Slider.value = PlayerMovementRuntimeData.CurrentEnergy.Value;
            if (PlayerMovementRuntimeData.IsEnergyDepleted.Value)
            {
                IsDepleted = true;
            }
            else
            {
                IsDepleted = false;
            }
        }

        public bool IsDepleted
        {
            get
            {
                return isDepleted;
            }

            set
            {
                isDepleted = value;
                if (isDepleted)
                {
                    sliderImage.color = Color.yellow;
                }
                else
                {
                    sliderImage.color = Color.green;
                }
            }
        }
        protected bool isDepleted = false;
    } 
}
