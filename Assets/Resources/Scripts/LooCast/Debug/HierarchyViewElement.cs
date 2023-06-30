using UnityEngine;
using UnityEngine.UI;
using LooCast.System;

public abstract class HierarchyViewElement : MonoBehaviour
{
    #region Fields
    [SerializeField] private Button expandButton;
    
    private bool expanded;
    #endregion

    #region Unity Callbacks
    private void Start()
    {
        expanded = false;
    }
    #endregion

    #region Methods
    public void ToggleExpanded()
    {
        if (expanded)
        {
            Collapse();
        }
        else
        {
            Expand();
        }
    }

    protected virtual void Expand()
    {
        expanded = true;
    }

    protected virtual void Collapse()
    {
        expanded = false;
    }
    #endregion
}
