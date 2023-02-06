using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.UI.Screen
{
    public class LoadGameScreen : Screen
    {
        private void Start()
        {
            isInitiallyVisible = false;
            isHideable = true;
            Initialize();
        }
    } 
}
