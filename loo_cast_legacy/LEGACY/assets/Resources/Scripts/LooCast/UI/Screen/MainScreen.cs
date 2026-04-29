using System.Collections;
using System.Collections.Generic;
using LooCast.UI.Canvas;
using UnityEngine;

namespace LooCast.UI.Screen
{
    public class MainScreen : Screen
    {
        private void Start()
        {
            isInitiallyVisible = true;
            isHideable = false;
            Initialize();
        }
    } 
}
