using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.UI.Bar
{
    public abstract class Bar : MonoBehaviour
    {
        public UnityEngine.UI.Slider Slider;

        public abstract void Refresh();
    } 
}
