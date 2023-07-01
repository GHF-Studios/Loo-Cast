using UnityEngine;
using UnityEngine.UI;
using LooCast.System;

public abstract class HierarchyViewElement : MonoBehaviour
{
    #region Fields
    [SerializeField] protected Text nameLabel;
    [SerializeField] protected Button expandButton;
    [SerializeField] protected GameObject elementContainerPanel;
    [SerializeField] protected GameObject elementContainer;

    private bool expanded;
    private bool initialized;
    protected bool hasAnyChildren;
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
        expandButton.transform.rotation = Quaternion.Euler(0, 0, 0);
        if (hasAnyChildren)
        {
            elementContainerPanel.SetActive(true);
        }
    }

    protected virtual void Collapse()
    {
        expanded = false;
        expandButton.transform.rotation = Quaternion.Euler(0, 0, 180);
        if (hasAnyChildren)
        {
            elementContainerPanel.SetActive(false); 
        }
    }

    protected virtual void InstantiateChildren()
    {
        
    }
    #endregion
}
