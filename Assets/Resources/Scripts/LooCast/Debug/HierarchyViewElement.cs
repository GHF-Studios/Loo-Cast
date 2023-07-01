using UnityEngine;
using UnityEngine.UI;
using LooCast.System;

public abstract class HierarchyViewElement : MonoBehaviour
{
    #region Fields
    [SerializeField] private Text nameLabel;
    [SerializeField] private Button expandButton;
    [SerializeField] protected GameObject elementContainerPanel;
    [SerializeField] protected GameObject elementContainer;

    private bool expanded;
    private bool initialized;
    #endregion

    #region Unity Callbacks
    private void Start()
    {
        expanded = false;
        initialized = false;
    }
    #endregion

    #region Methods
    public void Initialize(string name)
    {
        if (initialized)
        {
            throw new System.InvalidOperationException("HierarchyViewElement has already been initialized!");
        }
        
        nameLabel.text = name;
        expandButton.onClick.AddListener(ToggleExpanded);
        initialized = true;
    }

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
