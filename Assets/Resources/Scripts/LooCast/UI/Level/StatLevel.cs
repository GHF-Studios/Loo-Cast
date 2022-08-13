using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Level
{
    using Attribute.Stat;

    public class StatLevel : Level
    {
        public Stat Stat;

        private void Start()
        {
            Stat.Level.OnValueChanged.AddListener(() => { Refresh(); });
            Stat.MaxLevel.OnValueChanged.AddListener(() => { Refresh(); });
            Refresh();
        }

        public override void Refresh()
        {
            Text.text = $"{Stat.Level.Value}/{Stat.MaxLevel.Value}";
        }
    } 
}
