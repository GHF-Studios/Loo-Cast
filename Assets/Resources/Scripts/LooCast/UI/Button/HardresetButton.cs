using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.UI.Button
{
    using LooCast.Attribute;
    using LooCast.Attribute.Stat;

    public class HardresetButton : Button
    {
        [SerializeField]
        protected Screen.Screen screen;
        public Attributes Attributes;
        public Stats Stats;

        private void Start()
        {
            Initialize();
        }

        public void Initialize()
        {
            base.Initialize();
            screen = transform.GetComponentInParent<Screen.Screen>(true);
        }

        public override void OnClick()
        {
            PlayerPrefs.DeleteAll();
            Stats.Uncheat();
            Attributes.Uncheat();
            screen.Refresh();
        }
    }
}
