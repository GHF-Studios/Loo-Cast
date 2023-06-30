using UnityEngine;
using LooCast.System;
using System.Collections.Generic;

public class HierarchyViewFile : HierarchyViewElement
{
    #region Fields
    [SerializeField] private GameObject hierarchyViewObjectPrefab;
    [SerializeField] private GameObject objectContainer;

    private IFile hierarchyFile;
    private Dictionary<string, HierarchyViewObject> hierarchyViewObjectChildren;
    #endregion

    #region Methods
    public void Initialize(IFile hierarchyFile)
    {
        this.hierarchyFile = hierarchyFile;
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
