using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.UI.Bar
{
    using Weapon;

    public class CooldownBar : Bar
    { 
        public Weapon Weapon;

        public override void Refresh()
        {
            Slider.maxValue = Weapon.attackDelay;
            Slider.value = Weapon.attackTimer;
        }

        private void Update()
        {
            Refresh();
        }
    }
}
