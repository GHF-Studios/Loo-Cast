using UnityEngine;
using LooCast.System;
using System.Collections.Generic;
using UnityEngine.UI;

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
    protected override void InstantiateChildren()
    {
        base.InstantiateChildren();
        
        foreach (IObject _object in hierarchyObject.Children)
        {
            HierarchyViewObject hierarchyViewObject = Instantiate(hierarchyViewObjectPrefab, elementContainer.transform).GetComponent<HierarchyViewObject>();
            LayoutRebuilder.MarkLayoutForRebuild((RectTransform)transform);
            elementContainerLayoutGroup.CalculateLayoutInputHorizontal();
            elementContainerLayoutGroup.CalculateLayoutInputVertical();

            hierarchyViewObject.gameObject.name = _object.ObjectName;
            hierarchyViewObjectChildren.Add(_object.ObjectName, hierarchyViewObject);
            hierarchyViewObject.Initialize(_object);
            LayoutRebuilder.MarkLayoutForRebuild((RectTransform)transform);
            elementContainerLayoutGroup.CalculateLayoutInputHorizontal();
            elementContainerLayoutGroup.CalculateLayoutInputVertical();
        }

        if (hierarchyViewObjectChildren.Count != 0)
        {
            hasAnyChildren = true;
        }
    }
    #endregion
}
