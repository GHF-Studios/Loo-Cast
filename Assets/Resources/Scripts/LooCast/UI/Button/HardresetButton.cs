using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.UI.Button
{
    public class HardresetButton : Button
    {
        [SerializeField]
        protected Screen.Screen screen;

        public override void Initialize()
        {
            base.Initialize();
            screen = transform.GetComponentInParent<Screen.Screen>(true);
        }

        public override void OnClick()
        {
            PlayerPrefs.DeleteAll();
            screen.Refresh();
        }
    }
}
