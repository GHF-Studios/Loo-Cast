using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Bar
{
    using LooCast.Movement.Data;
    using LooCast.Movement.Data.Runtime;

    public class EnergyBar : Bar
    {
        public Image SliderImage { get; protected set; }
        public Image BorderImage { get; protected set; }
        public PlayerMovementRuntimeData PlayerMovementRuntimeData;

        private void Start()
        {
            SliderImage = transform.Find("SliderImage").GetComponent<Image>();
            BorderImage = transform.Find("BorderImage").GetComponent<Image>();
            PlayerMovementRuntimeData.MaxEnergy.OnValueChanged.AddListener(() => { Refresh(); });
            PlayerMovementRuntimeData.CurrentEnergy.OnValueChanged.AddListener(() => { Refresh(); });
            PlayerMovementRuntimeData.IsEnergyDepleted.OnValueChanged.AddListener(() => { Refresh(); });
        }

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
                    SliderImage.color = Color.yellow;
                }
                else
                {
                    SliderImage.color = Color.green;
                }
            }
        }
        protected bool isDepleted = false;
    } 
}
