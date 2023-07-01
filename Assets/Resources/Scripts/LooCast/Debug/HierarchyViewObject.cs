using UnityEngine;
using LooCast.System;
using System.Collections.Generic;

public class HierarchyViewObject : HierarchyViewElement
{
    #region Fields
    [SerializeField] private GameObject hierarchyViewObjectPrefab;

    private IObject hierarchyObject;
    private Dictionary<string, HierarchyViewObject> hierarchyViewObjectChildren;
    #endregion

    #region Methods
    public void Initialize(IObject hierarchyObject)
    {
        base.Initialize(hierarchyObject.ObjectName);
        
        this.hierarchyObject = hierarchyObject;
        hierarchyViewObjectChildren = new Dictionary<string, HierarchyViewObject>();
    }
    #endregion

    #region Overrides
    protected override void Expand()
    {
        base.Expand();
    }

    protected override void Collapse()
    {
        base.Collapse();
    }
    #endregion
}
