

global record BevyAPI
    
    
    record LuaAlignContent

        userdata
    
    --

        __tostring : function(LuaAlignContent):(string)

    end
    
    
    record LuaAlignItems

        userdata
    
    --

        __tostring : function(LuaAlignItems):(string)

    end
    
    
    record LuaAlignSelf

        userdata
    
    --

        __tostring : function(LuaAlignSelf):(string)

    end
    
    
    record LuaDirection

        userdata
    
    --

        __tostring : function(LuaDirection):(string)

    end
    
    
    record LuaFlexDirection

        userdata
    
    --

        __tostring : function(LuaFlexDirection):(string)

    end
    
    
    record LuaFlexWrap

        userdata
    
    --

        __tostring : function(LuaFlexWrap):(string)

    end
    
    
    record LuaFocusPolicy

        userdata
    
    --

        __tostring : function(LuaFocusPolicy):(string)

    end
    
    
    record LuaInteraction

        userdata
    
    --

        __tostring : function(LuaInteraction):(string)

    end
    
    
    record LuaJustifyContent

        userdata
    
    --

        __tostring : function(LuaJustifyContent):(string)

    end
    
    
    record LuaOverflow

        userdata
    
    --

        -- Whether to show or clip overflowing items on the x axis

        x : ReflectedValue

        -- Whether to show or clip overflowing items on the y axis

        y : ReflectedValue

        --Overflow is visible on both axes

        is_visible : function(LuaOverflow):(boolean)

        --Show overflowing items on both axes

        visible : function():(LuaOverflow)

        --Clip overflowing items on both axes

        clip : function():(LuaOverflow)

        --Clip overflowing items on the x axis

        clip_x : function():(LuaOverflow)

        --Clip overflowing items on the y axis

        clip_y : function():(LuaOverflow)

        __tostring : function(LuaOverflow):(string)

    end
    
    
    record LuaOverflowClass

        userdata
    
    --Collection of static methods for [`LuaOverflow`].

        --Show overflowing items on both axes

        visible : function():(LuaOverflow)

        --Clip overflowing items on both axes

        clip : function():(LuaOverflow)

        --Clip overflowing items on the x axis

        clip_x : function():(LuaOverflow)

        --Clip overflowing items on the y axis

        clip_y : function():(LuaOverflow)

    end
    
    
    record LuaPositionType

        userdata
    
    --

        __tostring : function(LuaPositionType):(string)

    end
    
    
    record LuaVal

        userdata
    
    --

        __tostring : function(LuaVal):(string)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaCalculatedClip

        userdata
    
    --

        -- The rect of the clip

        clip : LuaRect

        __tostring : function(LuaCalculatedClip):(string)

    end
    
    
    record LuaNode

        userdata
    
    --

        --The calculated node size as width and height in logical pixels
        --
        --automatically calculated by [`super::layout::ui_layout_system`]

        size : function(LuaNode):(LuaVec2)

        --Returns the size of the node in physical pixels based on the given scale factor and `UiScale`.

        physical_size : function(LuaNode,(number),(number)):(LuaVec2)

        --Returns the logical pixel coordinates of the UI node, based on its [`GlobalTransform`].

        logical_rect : function(LuaNode,LuaGlobalTransform):(LuaRect)

        --Returns the physical pixel coordinates of the UI node, based on its [`GlobalTransform`] and the scale factor.

        physical_rect : function(LuaNode,(LuaGlobalTransform),(number),(number)):(LuaRect)

        __tostring : function(LuaNode):(string)

    end
    
    
    record LuaStyle

        userdata
    
    --

        -- Which layout algorithm to use when laying out this node's contents:
        --
        --   - [`Display::Flex`]: Use the Flexbox layout algorithm
        --
        --   - [`Display::Grid`]: Use the CSS Grid layout algorithm
        --
        --   - [`Display::None`]: Hide this node and perform layout as if it does not exist.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/display>

        display : LuaDisplay

        -- Whether a node should be laid out in-flow with, or independently of it's siblings:
        --
        --  - [`PositionType::Relative`]: Layout this node in-flow with other nodes using the usual (flexbox/grid) layout algorithm.
        --
        --  - [`PositionType::Absolute`]: Layout this node on top and independently of other nodes.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/position>

        position_type : LuaPositionType

        -- Whether overflowing content should be displayed or clipped.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow>

        overflow : LuaOverflow

        -- Defines the text direction. For example English is written LTR (left-to-right) while Arabic is written RTL (right-to-left).
        --
        --
        --
        -- Note: the corresponding CSS property also affects box layout order, but this isn't yet implemented in bevy.
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/direction>

        direction : LuaDirection

        -- The horizontal position of the left edge of the node.
        --
        --  - For relatively positioned nodes, this is relative to the node's position as computed during regular layout.
        --
        --  - For absolutely positioned nodes, this is relative to the *parent* node's bounding box.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/left>

        left : LuaVal

        -- The horizontal position of the right edge of the node.
        --
        --  - For relatively positioned nodes, this is relative to the node's position as computed during regular layout.
        --
        --  - For absolutely positioned nodes, this is relative to the *parent* node's bounding box.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/right>

        right : LuaVal

        -- The vertical position of the top edge of the node.
        --
        --  - For relatively positioned nodes, this is relative to the node's position as computed during regular layout.
        --
        --  - For absolutely positioned nodes, this is relative to the *parent* node's bounding box.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/top>

        top : LuaVal

        -- The vertical position of the bottom edge of the node.
        --
        --  - For relatively positioned nodes, this is relative to the node's position as computed during regular layout.
        --
        --  - For absolutely positioned nodes, this is relative to the *parent* node's bounding box.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/bottom>

        bottom : LuaVal

        -- The ideal width of the node. `width` is used when it is within the bounds defined by `min_width` and `max_width`.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/width>

        width : LuaVal

        -- The ideal height of the node. `height` is used when it is within the bounds defined by `min_height` and `max_height`.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/height>

        height : LuaVal

        -- The minimum width of the node. `min_width` is used if it is greater than either `width` and/or `max_width`.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/min-width>

        min_width : LuaVal

        -- The minimum height of the node. `min_height` is used if it is greater than either `height` and/or `max_height`.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/min-height>

        min_height : LuaVal

        -- The maximum width of the node. `max_width` is used if it is within the bounds defined by `min_width` and `width`.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/max-width>

        max_width : LuaVal

        -- The maximum height of the node. `max_height` is used if it is within the bounds defined by `min_height` and `height`.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/max-height>

        max_height : LuaVal

        -- The aspect ratio of the node (defined as `width / height`)
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/aspect-ratio>

        aspect_ratio : ReflectedValue

        -- For Flexbox containers:
        --
        --   - Sets default cross-axis alignment of the child items.
        --
        -- For CSS Grid containers:
        --
        --   - Controls block (vertical) axis alignment of children of this grid container within their grid areas
        --
        --
        --
        -- This value is overriden [`JustifySelf`] on the child node is set.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/align-items>

        align_items : LuaAlignItems

        -- For Flexbox containers:
        --
        --   - This property has no effect. See `justify_content` for main-axis alignment of flex items.
        --
        -- For CSS Grid containers:
        --
        --   - Sets default inline (horizontal) axis alignment of child items within their grid areas
        --
        --
        --
        -- This value is overriden [`JustifySelf`] on the child node is set.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items>

        justify_items : ReflectedValue

        -- For Flexbox items:
        --
        --   - Controls cross-axis alignment of the item.
        --
        -- For CSS Grid items:
        --
        --   - Controls block (vertical) axis alignment of a grid item within it's grid area
        --
        --
        --
        -- If set to `Auto`, alignment is inherited from the value of [`AlignItems`] set on the parent node.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/align-self>

        align_self : LuaAlignSelf

        -- For Flexbox items:
        --
        --   - This property has no effect. See `justify_content` for main-axis alignment of flex items.
        --
        -- For CSS Grid items:
        --
        --   - Controls inline (horizontal) axis alignment of a grid item within it's grid area.
        --
        --
        --
        -- If set to `Auto`, alignment is inherited from the value of [`JustifyItems`] set on the parent node.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items>

        justify_self : ReflectedValue

        -- For Flexbox containers:
        --
        --   - Controls alignment of lines if flex_wrap is set to [`FlexWrap::Wrap`] and there are multiple lines of items
        --
        -- For CSS Grid container:
        --
        --   - Controls alignment of grid rows
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/align-content>

        align_content : LuaAlignContent

        -- For Flexbox containers:
        --
        --   - Controls alignment of items in the main axis
        --
        -- For CSS Grid containers:
        --
        --   - Controls alignment of grid columns
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content>

        justify_content : LuaJustifyContent

        -- The amount of space around a node outside its border.
        --
        --
        --
        -- If a percentage value is used, the percentage is calculated based on the width of the parent node.
        --
        --
        --
        -- # Example
        --
        -- ```
        --
        -- # use bevy_ui::{Style, UiRect, Val};
        --
        -- let style = Style {
        --
        --     margin: UiRect {
        --
        --         left: Val::Percent(10.),
        --
        --         right: Val::Percent(10.),
        --
        --         top: Val::Percent(15.),
        --
        --         bottom: Val::Percent(15.)
        --
        --     },
        --
        --     ..Default::default()
        --
        -- };
        --
        -- ```
        --
        -- A node with this style and a parent with dimensions of 100px by 300px, will have calculated margins of 10px on both left and right edges, and 15px on both top and bottom edges.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/margin>

        margin : ReflectedValue

        -- The amount of space between the edges of a node and its contents.
        --
        --
        --
        -- If a percentage value is used, the percentage is calculated based on the width of the parent node.
        --
        --
        --
        -- # Example
        --
        -- ```
        --
        -- # use bevy_ui::{Style, UiRect, Val};
        --
        -- let style = Style {
        --
        --     padding: UiRect {
        --
        --         left: Val::Percent(1.),
        --
        --         right: Val::Percent(2.),
        --
        --         top: Val::Percent(3.),
        --
        --         bottom: Val::Percent(4.)
        --
        --     },
        --
        --     ..Default::default()
        --
        -- };
        --
        -- ```
        --
        -- A node with this style and a parent with dimensions of 300px by 100px, will have calculated padding of 3px on the left, 6px on the right, 9px on the top and 12px on the bottom.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/padding>

        padding : ReflectedValue

        -- The amount of space between the margins of a node and its padding.
        --
        --
        --
        -- If a percentage value is used, the percentage is calculated based on the width of the parent node.
        --
        --
        --
        -- The size of the node will be expanded if there are constraints that prevent the layout algorithm from placing the border within the existing node boundary.
        --
        --
        --
        -- Rendering for borders is not yet implemented.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/border-width>

        border : ReflectedValue

        -- Whether a Flexbox container should be a row or a column. This property has no effect of Grid nodes.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction>

        flex_direction : LuaFlexDirection

        -- Whether a Flexbox container should wrap it's contents onto multiple line wrap if they overflow. This property has no effect of Grid nodes.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-wrap>

        flex_wrap : LuaFlexWrap

        -- Defines how much a flexbox item should grow if there's space available. Defaults to 0 (don't grow at all).
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-grow>

        flex_grow : number

        -- Defines how much a flexbox item should shrink if there's not enough space available. Defaults to 1.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-shrink>

        flex_shrink : number

        -- The initial length of a flexbox in the main axis, before flex growing/shrinking properties are applied.
        --
        --
        --
        -- `flex_basis` overrides `size` on the main axis if both are set,  but it obeys the bounds defined by `min_size` and `max_size`.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-basis>

        flex_basis : LuaVal

        -- The size of the gutters between items in a vertical flexbox layout or between rows in a grid layout
        --
        --
        --
        -- Note: Values of `Val::Auto` are not valid and are treated as zero.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/row-gap>

        row_gap : LuaVal

        -- The size of the gutters between items in a horizontal flexbox layout or between column in a grid layout
        --
        --
        --
        -- Note: Values of `Val::Auto` are not valid and are treated as zero.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/column-gap>

        column_gap : LuaVal

        -- Controls whether automatically placed grid items are placed row-wise or column-wise. And whether the sparse or dense packing algorithm is used.
        --
        -- Only affect Grid layouts
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow>

        grid_auto_flow : ReflectedValue

        -- Defines the number of rows a grid has and the sizes of those rows. If grid items are given explicit placements then more rows may
        --
        -- be implicitly generated by items that are placed out of bounds. The sizes of those rows are controlled by `grid_auto_rows` property.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-rows>

        grid_template_rows : ReflectedValue

        -- Defines the number of columns a grid has and the sizes of those columns. If grid items are given explicit placements then more columns may
        --
        -- be implicitly generated by items that are placed out of bounds. The sizes of those columns are controlled by `grid_auto_columns` property.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns>

        grid_template_columns : ReflectedValue

        -- Defines the size of implicitly created rows. Rows are created implicitly when grid items are given explicit placements that are out of bounds
        --
        -- of the rows explicitly created using `grid_template_rows`.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-rows>

        grid_auto_rows : ReflectedValue

        -- Defines the size of implicitly created columns. Columns are created implicitly when grid items are given explicit placements that are out of bounds
        --
        -- of the columns explicitly created using `grid_template_columms`.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns>

        grid_auto_columns : ReflectedValue

        -- The row in which a grid item starts and how many rows it spans.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-row>

        grid_row : ReflectedValue

        -- The column in which a grid item starts and how many columns it spans.
        --
        --
        --
        -- <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-column>

        grid_column : ReflectedValue

        __tostring : function(LuaStyle):(string)

    end
    
    
    record LuaUiImage

        userdata
    
    --

        -- Handle to the texture

        texture : ReflectedValue

        -- Whether the image should be flipped along its x-axis

        flip_x : boolean

        -- Whether the image should be flipped along its y-axis

        flip_y : boolean

        --flip the image along its x-axis

        with_flip_x : function(LuaUiImage):(LuaUiImage)

        --flip the image along its y-axis

        with_flip_y : function(LuaUiImage):(LuaUiImage)

        __tostring : function(LuaUiImage):(string)

    end
    
    
    record LuaUiImageClass

        userdata
    
    --Collection of static methods for [`LuaUiImage`].

    end
    
    
    record LuaButton

        userdata
    
    --

        __tostring : function(LuaButton):(string)

    end
    
    
    record LuaDisplay

        userdata
    
    --

        __tostring : function(LuaDisplay):(string)

    end
    
    
    record LuaAnimationPlayer

        userdata
    
    --

        --Is the animation paused

        is_paused : function(LuaAnimationPlayer):(boolean)

        --Speed of the animation playback

        speed : function(LuaAnimationPlayer):(number)

        --Time elapsed playing the animation

        elapsed : function(LuaAnimationPlayer):(number)

        --Pause the animation

        pause : function(LuaAnimationPlayer):()

        --Unpause the animation

        resume : function(LuaAnimationPlayer):()

    end
    
    
    record LuaName

        userdata
    
    --

        __tostring : function(LuaName):(string)

    end
    
    
    record LuaNameClass

        userdata
    
    --Collection of static methods for [`LuaName`].

    end
    
    
    record LuaGltfExtras

        userdata
    
    --

        value : string

        __tostring : function(LuaGltfExtras):(string)

    end
    
    
    record LuaChildren

        userdata
    
    --

        --Swaps the child at `a_index` with the child at `b_index`.

        swap : function(LuaChildren,(integer),(integer)):()

        __tostring : function(LuaChildren):(string)

    end
    
    
    record LuaParent

        userdata
    
    --

        --Gets the [`Entity`] ID of the parent.

        get : function(LuaParent):(LuaEntity)

        __tostring : function(LuaParent):(string)

    end
    
    
    record LuaText2dBounds

        userdata
    
    --

        size : LuaVec2

        __tostring : function(LuaText2dBounds):(string)

    end
    
    
    record LuaText

        userdata
    
    --

        sections : ReflectedValue

        -- The text's internal alignment.
        --
        -- Should not affect its position within a container.

        alignment : LuaTextAlignment

        -- How the text should linebreak when running out of the bounds determined by max_size

        linebreak_behavior : ReflectedValue

        --Returns this [`Text`] with a new [`TextAlignment`].

        with_alignment : function(LuaText,LuaTextAlignment):(LuaText)

        --Returns this [`Text`] with soft wrapping disabled.
        --
        --Hard wrapping, where text contains an explicit linebreak such as the escape sequence `\n`, will still occur.

        with_no_wrap : function(LuaText):(LuaText)

        __tostring : function(LuaText):(string)

    end
    
    
    record LuaTextClass

        userdata
    
    --Collection of static methods for [`LuaText`].

    end
    
    
    record LuaTextAlignment

        userdata
    
    --

        __tostring : function(LuaTextAlignment):(string)

    end
    
    
    record LuaTextSection

        userdata
    
    --

        value : string

        style : LuaTextStyle

        --Create an empty [`TextSection`] from a style. Useful when the value will be set dynamically.

        from_style : function(LuaTextStyle):(LuaTextSection)

        __tostring : function(LuaTextSection):(string)

    end
    
    
    record LuaTextSectionClass

        userdata
    
    --Collection of static methods for [`LuaTextSection`].

        --Create an empty [`TextSection`] from a style. Useful when the value will be set dynamically.

        from_style : function(LuaTextStyle):(LuaTextSection)

    end
    
    
    record LuaTextStyle

        userdata
    
    --

        font : ReflectedValue

        font_size : number

        color : LuaColor

        __tostring : function(LuaTextStyle):(string)

    end
    
    
    record LuaStopwatch

        userdata
    
    --

        --Returns the elapsed time since the last [`reset`](Stopwatch::reset)
        --
        --of the stopwatch, in seconds.
        --
        --
        --
        --# Examples
        --
        --```
        --
        --# use bevy_time::*;
        --
        --use std::time::Duration;
        --
        --let mut stopwatch = Stopwatch::new();
        --
        --stopwatch.tick(Duration::from_secs(1));
        --
        --assert_eq!(stopwatch.elapsed_secs(), 1.0);
        --
        --```
        --
        --
        --
        --# See Also
        --
        --
        --
        --[`elapsed`](Stopwatch::elapsed) - if a `Duration` is desirable instead.
        --
        --[`elapsed_secs_f64`](Stopwatch::elapsed_secs_f64) - if an `f64` is desirable instead.

        elapsed_secs : function(LuaStopwatch):(number)

        --Returns the elapsed time since the last [`reset`](Stopwatch::reset)
        --
        --of the stopwatch, in seconds, as f64.
        --
        --
        --
        --# See Also
        --
        --
        --
        --[`elapsed`](Stopwatch::elapsed) - if a `Duration` is desirable instead.
        --
        --[`elapsed_secs`](Stopwatch::elapsed_secs) - if an `f32` is desirable instead.

        elapsed_secs_f64 : function(LuaStopwatch):(number)

        --Returns `true` if the stopwatch is paused.
        --
        --
        --
        --# Examples
        --
        --```
        --
        --# use bevy_time::*;
        --
        --let mut stopwatch = Stopwatch::new();
        --
        --assert!(!stopwatch.paused());
        --
        --stopwatch.pause();
        --
        --assert!(stopwatch.paused());
        --
        --stopwatch.unpause();
        --
        --assert!(!stopwatch.paused());
        --
        --```

        paused : function(LuaStopwatch):(boolean)

        --Pauses the stopwatch. Any call to [`tick`](Stopwatch::tick) while
        --
        --paused will not have any effect on the elapsed time.
        --
        --
        --
        --# Examples
        --
        --```
        --
        --# use bevy_time::*;
        --
        --use std::time::Duration;
        --
        --let mut stopwatch = Stopwatch::new();
        --
        --stopwatch.pause();
        --
        --stopwatch.tick(Duration::from_secs_f32(1.5));
        --
        --assert!(stopwatch.paused());
        --
        --assert_eq!(stopwatch.elapsed_secs(), 0.0);
        --
        --```

        pause : function(LuaStopwatch):()

        --Unpauses the stopwatch. Resume the effect of ticking on elapsed time.
        --
        --
        --
        --# Examples
        --
        --```
        --
        --# use bevy_time::*;
        --
        --use std::time::Duration;
        --
        --let mut stopwatch = Stopwatch::new();
        --
        --stopwatch.pause();
        --
        --stopwatch.tick(Duration::from_secs_f32(1.0));
        --
        --stopwatch.unpause();
        --
        --stopwatch.tick(Duration::from_secs_f32(1.0));
        --
        --assert!(!stopwatch.paused());
        --
        --assert_eq!(stopwatch.elapsed_secs(), 1.0);
        --
        --```

        unpause : function(LuaStopwatch):()

        --Resets the stopwatch. The reset doesn't affect the paused state of the stopwatch.
        --
        --
        --
        --# Examples
        --
        --```
        --
        --# use bevy_time::*;
        --
        --use std::time::Duration;
        --
        --let mut stopwatch = Stopwatch::new();
        --
        --stopwatch.tick(Duration::from_secs_f32(1.5));
        --
        --stopwatch.reset();
        --
        --assert_eq!(stopwatch.elapsed_secs(), 0.0);
        --
        --```

        reset : function(LuaStopwatch):()

        --Create a new unpaused `Stopwatch` with no elapsed time.
        --
        --
        --
        --# Examples
        --
        --```
        --
        --# use bevy_time::*;
        --
        --let stopwatch = Stopwatch::new();
        --
        --assert_eq!(stopwatch.elapsed_secs(), 0.0);
        --
        --assert_eq!(stopwatch.paused(), false);
        --
        --```

        new : function():(LuaStopwatch)

        __tostring : function(LuaStopwatch):(string)

    end
    
    
    record LuaStopwatchClass

        userdata
    
    --Collection of static methods for [`LuaStopwatch`].

        --Create a new unpaused `Stopwatch` with no elapsed time.
        --
        --
        --
        --# Examples
        --
        --```
        --
        --# use bevy_time::*;
        --
        --let stopwatch = Stopwatch::new();
        --
        --assert_eq!(stopwatch.elapsed_secs(), 0.0);
        --
        --assert_eq!(stopwatch.paused(), false);
        --
        --```

        new : function():(LuaStopwatch)

    end
    
    
    record LuaTimer

        userdata
    
    --

        --Returns `true` if the timer has reached its duration at least once.
        --
        --See also [`Timer::just_finished`](Timer::just_finished).
        --
        --
        --
        --# Examples
        --
        --```
        --
        --# use bevy_time::*;
        --
        --use std::time::Duration;
        --
        --let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
        --
        --timer.tick(Duration::from_secs_f32(1.5));
        --
        --assert!(timer.finished());
        --
        --timer.tick(Duration::from_secs_f32(0.5));
        --
        --assert!(timer.finished());
        --
        --```

        finished : function(LuaTimer):(boolean)

        --Returns `true` only on the tick the timer reached its duration.
        --
        --
        --
        --# Examples
        --
        --```
        --
        --# use bevy_time::*;
        --
        --use std::time::Duration;
        --
        --let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
        --
        --timer.tick(Duration::from_secs_f32(1.5));
        --
        --assert!(timer.just_finished());
        --
        --timer.tick(Duration::from_secs_f32(0.5));
        --
        --assert!(!timer.just_finished());
        --
        --```

        just_finished : function(LuaTimer):(boolean)

        --Returns the time elapsed on the timer as an `f32`.
        --
        --See also [`Timer::elapsed`](Timer::elapsed).

        elapsed_secs : function(LuaTimer):(number)

        --Returns `true` if the timer is paused.
        --
        --
        --
        --See also [`Stopwatch::paused`](Stopwatch::paused).
        --
        --
        --
        --# Examples
        --
        --```
        --
        --# use bevy_time::*;
        --
        --let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
        --
        --assert!(!timer.paused());
        --
        --timer.pause();
        --
        --assert!(timer.paused());
        --
        --timer.unpause();
        --
        --assert!(!timer.paused());
        --
        --```

        paused : function(LuaTimer):(boolean)

        --Returns the percentage of the timer elapsed time (goes from 0.0 to 1.0).
        --
        --
        --
        --# Examples
        --
        --```
        --
        --# use bevy_time::*;
        --
        --use std::time::Duration;
        --
        --let mut timer = Timer::from_seconds(2.0, TimerMode::Once);
        --
        --timer.tick(Duration::from_secs_f32(0.5));
        --
        --assert_eq!(timer.percent(), 0.25);
        --
        --```

        percent : function(LuaTimer):(number)

        --Returns the percentage of the timer remaining time (goes from 1.0 to 0.0).
        --
        --
        --
        --# Examples
        --
        --```
        --
        --# use bevy_time::*;
        --
        --use std::time::Duration;
        --
        --let mut timer = Timer::from_seconds(2.0, TimerMode::Once);
        --
        --timer.tick(Duration::from_secs_f32(0.5));
        --
        --assert_eq!(timer.percent_left(), 0.75);
        --
        --```

        percent_left : function(LuaTimer):(number)

        --Returns the remaining time in seconds
        --
        --
        --
        --# Examples
        --
        --```
        --
        --# use bevy_time::*;
        --
        --use std::cmp::Ordering;
        --
        --use std::time::Duration;
        --
        --let mut timer = Timer::from_seconds(2.0, TimerMode::Once);
        --
        --timer.tick(Duration::from_secs_f32(0.5));
        --
        --let result = timer.remaining_secs().total_cmp(&1.5);
        --
        --assert_eq!(Ordering::Equal, result);
        --
        --```

        remaining_secs : function(LuaTimer):(number)

        --Returns the number of times a repeating timer
        --
        --finished during the last [`tick`](Timer<T>::tick) call.
        --
        --
        --
        --For non repeating-timers, this method will only ever
        --
        --return 0 or 1.
        --
        --
        --
        --# Examples
        --
        --```
        --
        --# use bevy_time::*;
        --
        --use std::time::Duration;
        --
        --let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);
        --
        --timer.tick(Duration::from_secs_f32(6.0));
        --
        --assert_eq!(timer.times_finished_this_tick(), 6);
        --
        --timer.tick(Duration::from_secs_f32(2.0));
        --
        --assert_eq!(timer.times_finished_this_tick(), 2);
        --
        --timer.tick(Duration::from_secs_f32(0.5));
        --
        --assert_eq!(timer.times_finished_this_tick(), 0);
        --
        --```

        times_finished_this_tick : function(LuaTimer):(integer)

        --Pauses the Timer. Disables the ticking of the timer.
        --
        --
        --
        --See also [`Stopwatch::pause`](Stopwatch::pause).
        --
        --
        --
        --# Examples
        --
        --```
        --
        --# use bevy_time::*;
        --
        --use std::time::Duration;
        --
        --let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
        --
        --timer.pause();
        --
        --timer.tick(Duration::from_secs_f32(0.5));
        --
        --assert_eq!(timer.elapsed_secs(), 0.0);
        --
        --```

        pause : function(LuaTimer):()

        --Unpauses the Timer. Resumes the ticking of the timer.
        --
        --
        --
        --See also [`Stopwatch::unpause()`](Stopwatch::unpause).
        --
        --
        --
        --# Examples
        --
        --```
        --
        --# use bevy_time::*;
        --
        --use std::time::Duration;
        --
        --let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
        --
        --timer.pause();
        --
        --timer.tick(Duration::from_secs_f32(0.5));
        --
        --timer.unpause();
        --
        --timer.tick(Duration::from_secs_f32(0.5));
        --
        --assert_eq!(timer.elapsed_secs(), 0.5);
        --
        --```

        unpause : function(LuaTimer):()

        --Resets the timer. The reset doesn't affect the `paused` state of the timer.
        --
        --
        --
        --See also [`Stopwatch::reset`](Stopwatch::reset).
        --
        --
        --
        --Examples
        --
        --```
        --
        --# use bevy_time::*;
        --
        --use std::time::Duration;
        --
        --let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
        --
        --timer.tick(Duration::from_secs_f32(1.5));
        --
        --timer.reset();
        --
        --assert!(!timer.finished());
        --
        --assert!(!timer.just_finished());
        --
        --assert_eq!(timer.elapsed_secs(), 0.0);
        --
        --```

        reset : function(LuaTimer):()

        __tostring : function(LuaTimer):(string)

    end
    
    
    record LuaTimerClass

        userdata
    
    --Collection of static methods for [`LuaTimer`].

    end
    
    
    record LuaEntity

        userdata
    
    --

        --Convert to a form convenient for passing outside of rust.
        --
        --
        --
        --Only useful for identifying entities within the same instance of an application. Do not use
        --
        --for serialization between runs.
        --
        --
        --
        --No particular structure is guaranteed for the returned bits.

        to_bits : function(LuaEntity):(integer)

        --Return a transiently unique identifier.
        --
        --
        --
        --No two simultaneously-live entities share the same index, but dead entities' indices may collide
        --
        --with both live and dead entities. Useful for compactly representing entities within a
        --
        --specific snapshot of the world, such as when serializing.

        index : function(LuaEntity):(integer)

        --Returns the generation of this Entity's index. The generation is incremented each time an
        --
        --entity with a given index is despawned. This serves as a "count" of the number of times a
        --
        --given index has been reused (index, generation) pairs uniquely identify a given Entity.

        generation : function(LuaEntity):(integer)

        --Creates a new entity ID with the specified `index` and a generation of 0.
        --
        --
        --
        --# Note
        --
        --
        --
        --Spawning a specific `entity` value is __rarely the right choice__. Most apps should favor
        --
        --[`Commands::spawn`](crate::system::Commands::spawn). This method should generally
        --
        --only be used for sharing entities across apps, and only when they have a scheme
        --
        --worked out to share an index space (which doesn't happen by default).
        --
        --
        --
        --In general, one should not try to synchronize the ECS by attempting to ensure that
        --
        --`Entity` lines up between instances, but instead insert a secondary identifier as
        --
        --a component.

        from_raw : function(integer):(LuaEntity)

        --Reconstruct an `Entity` previously destructured with [`Entity::to_bits`].
        --
        --
        --
        --Only useful when applied to results from `to_bits` in the same instance of an application.

        from_bits : function(integer):(LuaEntity)

        __tostring : function(LuaEntity):(string)

    end
    
    
    record LuaEntityClass

        userdata
    
    --Collection of static methods for [`LuaEntity`].

        --Creates a new entity ID with the specified `index` and a generation of 0.
        --
        --
        --
        --# Note
        --
        --
        --
        --Spawning a specific `entity` value is __rarely the right choice__. Most apps should favor
        --
        --[`Commands::spawn`](crate::system::Commands::spawn). This method should generally
        --
        --only be used for sharing entities across apps, and only when they have a scheme
        --
        --worked out to share an index space (which doesn't happen by default).
        --
        --
        --
        --In general, one should not try to synchronize the ECS by attempting to ensure that
        --
        --`Entity` lines up between instances, but instead insert a secondary identifier as
        --
        --a component.

        from_raw : function(integer):(LuaEntity)

        --Reconstruct an `Entity` previously destructured with [`Entity::to_bits`].
        --
        --
        --
        --Only useful when applied to results from `to_bits` in the same instance of an application.

        from_bits : function(integer):(LuaEntity)

    end
    
    
    record LuaTransform

        userdata
    
    --

        -- Position of the entity. In 2d, the last value of the `Vec3` is used for z-ordering.
        --
        --
        --
        -- See the [`translations`] example for usage.
        --
        --
        --
        -- [`translations`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/translation.rs

        translation : LuaVec3

        -- Rotation of the entity.
        --
        --
        --
        -- See the [`3d_rotation`] example for usage.
        --
        --
        --
        -- [`3d_rotation`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/3d_rotation.rs

        rotation : LuaQuat

        -- Scale of the entity.
        --
        --
        --
        -- See the [`scale`] example for usage.
        --
        --
        --
        -- [`scale`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/scale.rs

        scale : LuaVec3

        --Returns this [`Transform`] with a new rotation so that [`Transform::forward`]
        --
        --points towards the `target` position and [`Transform::up`] points towards `up`.
        --
        --
        --
        --In some cases it's not possible to construct a rotation. Another axis will be picked in those cases:
        --
        --* if `target` is the same as the transform translation, `Vec3::Z` is used instead
        --
        --* if `up` is zero, `Vec3::Y` is used instead
        --
        --* if the resulting forward direction is parallel with `up`, an orthogonal vector is used as the "right" direction

        looking_at : function(LuaTransform,(LuaVec3),(LuaVec3)):(LuaTransform)

        --Returns this [`Transform`] with a new rotation so that [`Transform::forward`]
        --
        --points in the given `direction` and [`Transform::up`] points towards `up`.
        --
        --
        --
        --In some cases it's not possible to construct a rotation. Another axis will be picked in those cases:
        --
        --* if `direction` is zero, `Vec3::Z` is used instead
        --
        --* if `up` is zero, `Vec3::Y` is used instead
        --
        --* if `direction` is parallel with `up`, an orthogonal vector is used as the "right" direction

        looking_to : function(LuaTransform,(LuaVec3),(LuaVec3)):(LuaTransform)

        --Returns this [`Transform`] with a new translation.

        with_translation : function(LuaTransform,LuaVec3):(LuaTransform)

        --Returns this [`Transform`] with a new rotation.

        with_rotation : function(LuaTransform,LuaQuat):(LuaTransform)

        --Returns this [`Transform`] with a new scale.

        with_scale : function(LuaTransform,LuaVec3):(LuaTransform)

        --Returns the 3d affine transformation matrix from this transforms translation,
        --
        --rotation, and scale.

        compute_matrix : function(LuaTransform):(LuaMat4)

        --Returns the 3d affine transformation matrix from this transforms translation,
        --
        --rotation, and scale.

        compute_affine : function(LuaTransform):(LuaAffine3A)

        --Get the unit vector in the local `X` direction.

        local_x : function(LuaTransform):(LuaVec3)

        --Equivalent to [`-local_x()`][Transform::local_x()]

        left : function(LuaTransform):(LuaVec3)

        --Equivalent to [`local_x()`][Transform::local_x()]

        right : function(LuaTransform):(LuaVec3)

        --Get the unit vector in the local `Y` direction.

        local_y : function(LuaTransform):(LuaVec3)

        --Equivalent to [`local_y()`][Transform::local_y]

        up : function(LuaTransform):(LuaVec3)

        --Equivalent to [`-local_y()`][Transform::local_y]

        down : function(LuaTransform):(LuaVec3)

        --Get the unit vector in the local `Z` direction.

        local_z : function(LuaTransform):(LuaVec3)

        --Equivalent to [`-local_z()`][Transform::local_z]

        forward : function(LuaTransform):(LuaVec3)

        --Equivalent to [`local_z()`][Transform::local_z]

        back : function(LuaTransform):(LuaVec3)

        --Multiplies `self` with `transform` component by component, returning the
        --
        --resulting [`Transform`]

        mul_transform : function(LuaTransform,LuaTransform):(LuaTransform)

        --Transforms the given `point`, applying scale, rotation and translation.
        --
        --
        --
        --If this [`Transform`] has a parent, this will transform a `point` that is
        --
        --relative to the parent's [`Transform`] into one relative to this [`Transform`].
        --
        --
        --
        --If this [`Transform`] does not have a parent, this will transform a `point`
        --
        --that is in global space into one relative to this [`Transform`].
        --
        --
        --
        --If you want to transform a `point` in global space to the local space of this [`Transform`],
        --
        --consider using [`GlobalTransform::transform_point()`] instead.

        transform_point : function(LuaTransform,LuaVec3):(LuaVec3)

        --Rotates this [`Transform`] by the given rotation.
        --
        --
        --
        --If this [`Transform`] has a parent, the `rotation` is relative to the rotation of the parent.
        --
        --
        --
        --# Examples
        --
        --
        --
        --- [`3d_rotation`]
        --
        --
        --
        --[`3d_rotation`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/3d_rotation.rs

        rotate : function(LuaTransform,LuaQuat):()

        --Rotates this [`Transform`] around the given `axis` by `angle` (in radians).
        --
        --
        --
        --If this [`Transform`] has a parent, the `axis` is relative to the rotation of the parent.

        rotate_axis : function(LuaTransform,(LuaVec3),(number)):()

        --Rotates this [`Transform`] around the `X` axis by `angle` (in radians).
        --
        --
        --
        --If this [`Transform`] has a parent, the axis is relative to the rotation of the parent.

        rotate_x : function(LuaTransform,number):()

        --Rotates this [`Transform`] around the `Y` axis by `angle` (in radians).
        --
        --
        --
        --If this [`Transform`] has a parent, the axis is relative to the rotation of the parent.

        rotate_y : function(LuaTransform,number):()

        --Rotates this [`Transform`] around the `Z` axis by `angle` (in radians).
        --
        --
        --
        --If this [`Transform`] has a parent, the axis is relative to the rotation of the parent.

        rotate_z : function(LuaTransform,number):()

        --Rotates this [`Transform`] by the given `rotation`.
        --
        --
        --
        --The `rotation` is relative to this [`Transform`]'s current rotation.

        rotate_local : function(LuaTransform,LuaQuat):()

        --Rotates this [`Transform`] around its local `axis` by `angle` (in radians).

        rotate_local_axis : function(LuaTransform,(LuaVec3),(number)):()

        --Rotates this [`Transform`] around its local `X` axis by `angle` (in radians).

        rotate_local_x : function(LuaTransform,number):()

        --Rotates this [`Transform`] around its local `Y` axis by `angle` (in radians).

        rotate_local_y : function(LuaTransform,number):()

        --Rotates this [`Transform`] around its local `Z` axis by `angle` (in radians).

        rotate_local_z : function(LuaTransform,number):()

        --Translates this [`Transform`] around a `point` in space.
        --
        --
        --
        --If this [`Transform`] has a parent, the `point` is relative to the [`Transform`] of the parent.

        translate_around : function(LuaTransform,(LuaVec3),(LuaQuat)):()

        --Rotates this [`Transform`] around a `point` in space.
        --
        --
        --
        --If this [`Transform`] has a parent, the `point` is relative to the [`Transform`] of the parent.

        rotate_around : function(LuaTransform,(LuaVec3),(LuaQuat)):()

        --Rotates this [`Transform`] so that [`Transform::forward`] points towards the `target` position,
        --
        --and [`Transform::up`] points towards `up`.
        --
        --
        --
        --In some cases it's not possible to construct a rotation. Another axis will be picked in those cases:
        --
        --* if `target` is the same as the transtorm translation, `Vec3::Z` is used instead
        --
        --* if `up` is zero, `Vec3::Y` is used instead
        --
        --* if the resulting forward direction is parallel with `up`, an orthogonal vector is used as the "right" direction

        look_at : function(LuaTransform,(LuaVec3),(LuaVec3)):()

        --Rotates this [`Transform`] so that [`Transform::forward`] points in the given `direction`
        --
        --and [`Transform::up`] points towards `up`.
        --
        --
        --
        --In some cases it's not possible to construct a rotation. Another axis will be picked in those cases:
        --
        --* if `direction` is zero, `Vec3::NEG_Z` is used instead
        --
        --* if `up` is zero, `Vec3::Y` is used instead
        --
        --* if `direction` is parallel with `up`, an orthogonal vector is used as the "right" direction

        look_to : function(LuaTransform,(LuaVec3),(LuaVec3)):()

        --Creates a new [`Transform`] at the position `(x, y, z)`. In 2d, the `z` component
        --
        --is used for z-ordering elements: higher `z`-value will be in front of lower
        --
        --`z`-value.

        from_xyz : function((number),(number),(number)):(LuaTransform)

        --Extracts the translation, rotation, and scale from `matrix`. It must be a 3d affine
        --
        --transformation matrix.

        from_matrix : function(LuaMat4):(LuaTransform)

        --Creates a new [`Transform`], with `translation`. Rotation will be 0 and scale 1 on
        --
        --all axes.

        from_translation : function(LuaVec3):(LuaTransform)

        --Creates a new [`Transform`], with `rotation`. Translation will be 0 and scale 1 on
        --
        --all axes.

        from_rotation : function(LuaQuat):(LuaTransform)

        --Creates a new [`Transform`], with `scale`. Translation will be 0 and rotation 0 on
        --
        --all axes.

        from_scale : function(LuaVec3):(LuaTransform)

        __tostring : function(LuaTransform):(string)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaTransformClass

        userdata
    
    --Collection of static methods for [`LuaTransform`].

        --Creates a new [`Transform`] at the position `(x, y, z)`. In 2d, the `z` component
        --
        --is used for z-ordering elements: higher `z`-value will be in front of lower
        --
        --`z`-value.

        from_xyz : function((number),(number),(number)):(LuaTransform)

        --Extracts the translation, rotation, and scale from `matrix`. It must be a 3d affine
        --
        --transformation matrix.

        from_matrix : function(LuaMat4):(LuaTransform)

        --Creates a new [`Transform`], with `translation`. Rotation will be 0 and scale 1 on
        --
        --all axes.

        from_translation : function(LuaVec3):(LuaTransform)

        --Creates a new [`Transform`], with `rotation`. Translation will be 0 and scale 1 on
        --
        --all axes.

        from_rotation : function(LuaQuat):(LuaTransform)

        --Creates a new [`Transform`], with `scale`. Translation will be 0 and rotation 0 on
        --
        --all axes.

        from_scale : function(LuaVec3):(LuaTransform)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaGlobalTransform

        userdata
    
    --

        --Returns the 3d affine transformation matrix as a [`Mat4`].

        compute_matrix : function(LuaGlobalTransform):(LuaMat4)

        --Returns the 3d affine transformation matrix as an [`Affine3A`].

        affine : function(LuaGlobalTransform):(LuaAffine3A)

        --Returns the transformation as a [`Transform`].
        --
        --
        --
        --The transform is expected to be non-degenerate and without shearing, or the output
        --
        --will be invalid.

        compute_transform : function(LuaGlobalTransform):(LuaTransform)

        --Returns the [`Transform`] `self` would have if it was a child of an entity
        --
        --with the `parent` [`GlobalTransform`].
        --
        --
        --
        --This is useful if you want to "reparent" an [`Entity`](bevy_ecs::entity::Entity).
        --
        --Say you have an entity `e1` that you want to turn into a child of `e2`,
        --
        --but you want `e1` to keep the same global transform, even after re-parenting. You would use:
        --
        --
        --
        --```rust
        --
        --# use bevy_transform::prelude::{GlobalTransform, Transform};
        --
        --# use bevy_ecs::prelude::{Entity, Query, Component, Commands};
        --
        --# use bevy_hierarchy::{prelude::Parent, BuildChildren};
        --
        --#[derive(Component)]
        --
        --struct ToReparent {
        --
        --    new_parent: Entity,
        --
        --}
        --
        --fn reparent_system(
        --
        --    mut commands: Commands,
        --
        --    mut targets: Query<(&mut Transform, Entity, &GlobalTransform, &ToReparent)>,
        --
        --    transforms: Query<&GlobalTransform>,
        --
        --) {
        --
        --    for (mut transform, entity, initial, to_reparent) in targets.iter_mut() {
        --
        --        if let Ok(parent_transform) = transforms.get(to_reparent.new_parent) {
        --
        --            *transform = initial.reparented_to(parent_transform);
        --
        --            commands.entity(entity)
        --
        --                .remove::<ToReparent>()
        --
        --                .set_parent(to_reparent.new_parent);
        --
        --        }
        --
        --    }
        --
        --}
        --
        --```
        --
        --
        --
        --The transform is expected to be non-degenerate and without shearing, or the output
        --
        --will be invalid.

        reparented_to : function(LuaGlobalTransform,LuaGlobalTransform):(LuaTransform)

        --Return the local right vector (X).

        right : function(LuaGlobalTransform):(LuaVec3)

        --Return the local left vector (-X).

        left : function(LuaGlobalTransform):(LuaVec3)

        --Return the local up vector (Y).

        up : function(LuaGlobalTransform):(LuaVec3)

        --Return the local down vector (-Y).

        down : function(LuaGlobalTransform):(LuaVec3)

        --Return the local back vector (Z).

        back : function(LuaGlobalTransform):(LuaVec3)

        --Return the local forward vector (-Z).

        forward : function(LuaGlobalTransform):(LuaVec3)

        --Get the translation as a [`Vec3`].

        translation : function(LuaGlobalTransform):(LuaVec3)

        --Get the translation as a [`Vec3A`].

        translation_vec3a : function(LuaGlobalTransform):(LuaVec3A)

        --Get an upper bound of the radius from the given `extents`.

        radius_vec3a : function(LuaGlobalTransform,LuaVec3A):(number)

        --Transforms the given `point`, applying shear, scale, rotation and translation.
        --
        --
        --
        --This moves `point` into the local space of this [`GlobalTransform`].

        transform_point : function(LuaGlobalTransform,LuaVec3):(LuaVec3)

        --Multiplies `self` with `transform` component by component, returning the
        --
        --resulting [`GlobalTransform`]

        mul_transform : function(LuaGlobalTransform,LuaTransform):(LuaGlobalTransform)

        __tostring : function(LuaGlobalTransform):(string)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaAmbientLight

        userdata
    
    --

        color : LuaColor

        -- A direct scale factor multiplied with `color` before being passed to the shader.

        brightness : number

        __tostring : function(LuaAmbientLight):(string)

    end
    
    
    record LuaCubemapVisibleEntities

        userdata
    
    --

        __tostring : function(LuaCubemapVisibleEntities):(string)

    end
    
    
    record LuaDirectionalLight

        userdata
    
    --

        color : LuaColor

        -- Illuminance in lux

        illuminance : number

        shadows_enabled : boolean

        shadow_depth_bias : number

        -- A bias applied along the direction of the fragment's surface normal. It is scaled to the
        --
        -- shadow map's texel size so that it is automatically adjusted to the orthographic projection.

        shadow_normal_bias : number

        __tostring : function(LuaDirectionalLight):(string)

    end
    
    
    record LuaDirectionalLightShadowMap

        userdata
    
    --

        size : integer

        __tostring : function(LuaDirectionalLightShadowMap):(string)

    end
    
    
    record LuaNotShadowCaster

        userdata
    
    --

    end
    
    
    record LuaNotShadowReceiver

        userdata
    
    --

    end
    
    
    record LuaPointLight

        userdata
    
    --

        color : LuaColor

        intensity : number

        range : number

        radius : number

        shadows_enabled : boolean

        shadow_depth_bias : number

        -- A bias applied along the direction of the fragment's surface normal. It is scaled to the
        --
        -- shadow map's texel size so that it can be small close to the camera and gets larger further
        --
        -- away.

        shadow_normal_bias : number

        __tostring : function(LuaPointLight):(string)

    end
    
    
    record LuaPointLightShadowMap

        userdata
    
    --

        size : integer

        __tostring : function(LuaPointLightShadowMap):(string)

    end
    
    
    record LuaAlphaMode

        userdata
    
    --

        __tostring : function(LuaAlphaMode):(string)

    end
    
    
    record LuaWireframe

        userdata
    
    --

        __tostring : function(LuaWireframe):(string)

    end
    
    
    record LuaWireframeConfig

        userdata
    
    --

        -- Whether to show wireframes for all meshes. If `false`, only meshes with a [Wireframe] component will be rendered.

        global : boolean

        __tostring : function(LuaWireframeConfig):(string)

    end
    
    
    record LuaCamera3dDepthLoadOp

        userdata
    
    --

        __tostring : function(LuaCamera3dDepthLoadOp):(string)

    end
    
    
    record LuaClearColor

        userdata
    
    --

        __tostring : function(LuaClearColor):(string)

    end
    
    
    record LuaClearColorConfig

        userdata
    
    --

        __tostring : function(LuaClearColorConfig):(string)

    end
    
    
    record LuaCamera2d

        userdata
    
    --

        clear_color : LuaClearColorConfig

    end
    
    
    record LuaCamera3d

        userdata
    
    --

        -- The clear color operation to perform for the main 3d pass.

        clear_color : LuaClearColorConfig

        -- The depth clear operation to perform for the main 3d pass.

        depth_load_op : LuaCamera3dDepthLoadOp

        -- The texture usages for the depth texture created for the main 3d pass.

        depth_texture_usages : ReflectedValue

    end
    
    
    record LuaAnchor

        userdata
    
    --

        as_vec : function(LuaAnchor):(LuaVec2)

        __tostring : function(LuaAnchor):(string)

    end
    
    
    record LuaMesh2dHandle

        userdata
    
    --

        __tostring : function(LuaMesh2dHandle):(string)

    end
    
    
    record LuaTextureAtlasSprite

        userdata
    
    --

        -- The tint color used to draw the sprite, defaulting to [`Color::WHITE`]

        color : LuaColor

        -- Texture index in [`TextureAtlas`]

        index : integer

        -- Whether to flip the sprite in the X axis

        flip_x : boolean

        -- Whether to flip the sprite in the Y axis

        flip_y : boolean

        -- An optional custom size for the sprite that will be used when rendering, instead of the size
        --
        -- of the sprite's image in the atlas

        custom_size : ReflectedValue

        -- [`Anchor`] point of the sprite in the world

        anchor : LuaAnchor

        --Create a new [`TextureAtlasSprite`] with a sprite index,
        --
        --it should be valid in the corresponding [`TextureAtlas`]

        new : function(integer):(LuaTextureAtlasSprite)

        __tostring : function(LuaTextureAtlasSprite):(string)

    end
    
    
    record LuaTextureAtlasSpriteClass

        userdata
    
    --Collection of static methods for [`LuaTextureAtlasSprite`].

        --Create a new [`TextureAtlasSprite`] with a sprite index,
        --
        --it should be valid in the corresponding [`TextureAtlas`]

        new : function(integer):(LuaTextureAtlasSprite)

    end
    
    
    record LuaSprite

        userdata
    
    --

        -- The sprite's color tint

        color : LuaColor

        -- Flip the sprite along the `X` axis

        flip_x : boolean

        -- Flip the sprite along the `Y` axis

        flip_y : boolean

        -- An optional custom size for the sprite that will be used when rendering, instead of the size
        --
        -- of the sprite's image

        custom_size : ReflectedValue

        -- An optional rectangle representing the region of the sprite's image to render, instead of
        --
        -- rendering the full image. This is an easy one-off alternative to using a texture atlas.

        rect : ReflectedValue

        -- [`Anchor`] point of the sprite in the world

        anchor : LuaAnchor

        __tostring : function(LuaSprite):(string)

    end
    
    
    record LuaRenderLayers

        userdata
    
    --

        --Determine if a `RenderLayers` intersects another.
        --
        --
        --
        --`RenderLayers`s intersect if they share any common layers.
        --
        --
        --
        --A `RenderLayers` with no layers will not match any other
        --
        --`RenderLayers`, even another with no layers.

        intersects : function(LuaRenderLayers,LuaRenderLayers):(boolean)

        --Create a new `RenderLayers` that belongs to all layers.

        all : function():(LuaRenderLayers)

        --Create a new `RenderLayers` that belongs to no layers.

        none : function():(LuaRenderLayers)

        __tostring : function(LuaRenderLayers):(string)

    end
    
    
    record LuaRenderLayersClass

        userdata
    
    --Collection of static methods for [`LuaRenderLayers`].

        --Create a new `RenderLayers` that belongs to all layers.

        all : function():(LuaRenderLayers)

        --Create a new `RenderLayers` that belongs to no layers.

        none : function():(LuaRenderLayers)

    end
    
    
    record LuaVisibility

        userdata
    
    --

        __tostring : function(LuaVisibility):(string)

    end
    
    
    record LuaVisibleEntities

        userdata
    
    --

        len : function(LuaVisibleEntities):(integer)

        is_empty : function(LuaVisibleEntities):(boolean)

        __tostring : function(LuaVisibleEntities):(string)

    end
    
    
    record LuaComputedVisibility

        userdata
    
    --

        --Whether this entity is visible to something this frame. This is true if and only if [`Self::is_visible_in_hierarchy`] and [`Self::is_visible_in_view`]
        --
        --are true. This is the canonical method to call to determine if an entity should be drawn.
        --
        --This value is updated in [`PostUpdate`] by the [`VisibilitySystems::CheckVisibility`] system set.
        --
        --Reading it during [`Update`](bevy_app::Update) will yield the value from the previous frame.

        is_visible : function(LuaComputedVisibility):(boolean)

        --Whether this entity is visible in the entity hierarchy, which is determined by the [`Visibility`] component.
        --
        --This takes into account "visibility inheritance". If any of this entity's ancestors (see [`Parent`]) are hidden, this entity
        --
        --will be hidden as well. This value is updated in the [`VisibilitySystems::VisibilityPropagate`], which lives in the [`PostUpdate`] schedule.

        is_visible_in_hierarchy : function(LuaComputedVisibility):(boolean)

        --Whether this entity is visible in _any_ view (Cameras, Lights, etc). Each entity type (and view type) should choose how to set this
        --
        --value. For cameras and drawn entities, this will take into account [`RenderLayers`].
        --
        --
        --
        --This value is reset to `false` every frame in [`VisibilitySystems::VisibilityPropagate`] during [`PostUpdate`].
        --
        --Each entity type then chooses how to set this field in the [`VisibilitySystems::CheckVisibility`] system set, in [`PostUpdate`].
        --
        --Meshes might use frustum culling to decide if they are visible in a view.
        --
        --Other entities might just set this to `true` every frame.

        is_visible_in_view : function(LuaComputedVisibility):(boolean)

        --Sets `is_visible_in_view` to `true`. This is not reversible for a given frame, as it encodes whether or not this is visible in
        --
        --_any_ view. This will be automatically reset to `false` every frame in [`VisibilitySystems::VisibilityPropagate`] and then set
        --
        --to the proper value in [`VisibilitySystems::CheckVisibility`]. This should _only_ be set in systems with the [`VisibilitySystems::CheckVisibility`]
        --
        --label. Don't call this unless you are defining a custom visibility system. For normal user-defined entity visibility, see [`Visibility`].

        set_visible_in_view : function(LuaComputedVisibility):()

        __tostring : function(LuaComputedVisibility):(string)

    end
    
    
    record LuaSkinnedMesh

        userdata
    
    --

        inverse_bindposes : ReflectedValue

        joints : ReflectedValue

        __tostring : function(LuaSkinnedMesh):(string)

    end
    
    
    record LuaScalingMode

        userdata
    
    --

        __tostring : function(LuaScalingMode):(string)

    end
    
    
    record LuaColor

        userdata
    
    --

        --Get red in sRGB colorspace.

        r : function(LuaColor):(number)

        --Get green in sRGB colorspace.

        g : function(LuaColor):(number)

        --Get blue in sRGB colorspace.

        b : function(LuaColor):(number)

        --Returns this color with red set to a new value in sRGB colorspace.

        with_r : function(LuaColor,number):(LuaColor)

        --Returns this color with green set to a new value in sRGB colorspace.

        with_g : function(LuaColor,number):(LuaColor)

        --Returns this color with blue set to a new value in sRGB colorspace.

        with_b : function(LuaColor,number):(LuaColor)

        --Get alpha.

        a : function(LuaColor):(number)

        --Returns this color with a new alpha value.

        with_a : function(LuaColor,number):(LuaColor)

        --Converts a `Color` to variant `Color::Rgba`

        as_rgba : function(LuaColor):(LuaColor)

        --Converts a `Color` to variant `Color::RgbaLinear`

        as_rgba_linear : function(LuaColor):(LuaColor)

        --Converts a `Color` to variant `Color::Hsla`

        as_hsla : function(LuaColor):(LuaColor)

        --Converts a `Color` to variant `Color::Lcha`

        as_lcha : function(LuaColor):(LuaColor)

        --Converts `Color` to a `u32` from sRGB colorspace.
        --
        --
        --
        --Maps the RGBA channels in RGBA order to a little-endian byte array (GPUs are little-endian).
        --
        --`A` will be the most significant byte and `R` the least significant.

        as_rgba_u32 : function(LuaColor):(integer)

        --Converts Color to a u32 from linear RGB colorspace.
        --
        --
        --
        --Maps the RGBA channels in RGBA order to a little-endian byte array (GPUs are little-endian).
        --
        --`A` will be the most significant byte and `R` the least significant.

        as_linear_rgba_u32 : function(LuaColor):(integer)

        --New `Color` from sRGB colorspace.
        --
        --
        --
        --# Arguments
        --
        --
        --
        --* `r` - Red channel. [0.0, 1.0]
        --
        --* `g` - Green channel. [0.0, 1.0]
        --
        --* `b` - Blue channel. [0.0, 1.0]
        --
        --
        --
        --See also [`Color::rgba`], [`Color::rgb_u8`], [`Color::hex`].

        rgb : function((number),(number),(number)):(LuaColor)

        --New `Color` from sRGB colorspace.
        --
        --
        --
        --# Arguments
        --
        --
        --
        --* `r` - Red channel. [0.0, 1.0]
        --
        --* `g` - Green channel. [0.0, 1.0]
        --
        --* `b` - Blue channel. [0.0, 1.0]
        --
        --* `a` - Alpha channel. [0.0, 1.0]
        --
        --
        --
        --See also [`Color::rgb`], [`Color::rgba_u8`], [`Color::hex`].

        rgba : function((number),(number),(number),(number)):(LuaColor)

        --New `Color` from linear RGB colorspace.
        --
        --
        --
        --# Arguments
        --
        --
        --
        --* `r` - Red channel. [0.0, 1.0]
        --
        --* `g` - Green channel. [0.0, 1.0]
        --
        --* `b` - Blue channel. [0.0, 1.0]
        --
        --
        --
        --See also [`Color::rgb`], [`Color::rgba_linear`].

        rgb_linear : function((number),(number),(number)):(LuaColor)

        --New `Color` from linear RGB colorspace.
        --
        --
        --
        --# Arguments
        --
        --
        --
        --* `r` - Red channel. [0.0, 1.0]
        --
        --* `g` - Green channel. [0.0, 1.0]
        --
        --* `b` - Blue channel. [0.0, 1.0]
        --
        --* `a` - Alpha channel. [0.0, 1.0]
        --
        --
        --
        --See also [`Color::rgba`], [`Color::rgb_linear`].

        rgba_linear : function((number),(number),(number),(number)):(LuaColor)

        --New `Color` with HSL representation in sRGB colorspace.
        --
        --
        --
        --# Arguments
        --
        --
        --
        --* `hue` - Hue channel. [0.0, 360.0]
        --
        --* `saturation` - Saturation channel. [0.0, 1.0]
        --
        --* `lightness` - Lightness channel. [0.0, 1.0]
        --
        --
        --
        --See also [`Color::hsla`].

        hsl : function((number),(number),(number)):(LuaColor)

        --New `Color` with HSL representation in sRGB colorspace.
        --
        --
        --
        --# Arguments
        --
        --
        --
        --* `hue` - Hue channel. [0.0, 360.0]
        --
        --* `saturation` - Saturation channel. [0.0, 1.0]
        --
        --* `lightness` - Lightness channel. [0.0, 1.0]
        --
        --* `alpha` - Alpha channel. [0.0, 1.0]
        --
        --
        --
        --See also [`Color::hsl`].

        hsla : function((number),(number),(number),(number)):(LuaColor)

        --New `Color` with LCH representation in sRGB colorspace.
        --
        --
        --
        --# Arguments
        --
        --
        --
        --* `lightness` - Lightness channel. [0.0, 1.5]
        --
        --* `chroma` - Chroma channel. [0.0, 1.5]
        --
        --* `hue` - Hue channel. [0.0, 360.0]
        --
        --
        --
        --See also [`Color::lcha`].

        lch : function((number),(number),(number)):(LuaColor)

        --New `Color` with LCH representation in sRGB colorspace.
        --
        --
        --
        --# Arguments
        --
        --
        --
        --* `lightness` - Lightness channel. [0.0, 1.5]
        --
        --* `chroma` - Chroma channel. [0.0, 1.5]
        --
        --* `hue` - Hue channel. [0.0, 360.0]
        --
        --* `alpha` - Alpha channel. [0.0, 1.0]
        --
        --
        --
        --See also [`Color::lch`].

        lcha : function((number),(number),(number),(number)):(LuaColor)

        --New `Color` from sRGB colorspace.
        --
        --
        --
        --# Arguments
        --
        --
        --
        --* `r` - Red channel. [0, 255]
        --
        --* `g` - Green channel. [0, 255]
        --
        --* `b` - Blue channel. [0, 255]
        --
        --
        --
        --See also [`Color::rgb`], [`Color::rgba_u8`], [`Color::hex`].

        rgb_u8 : function((integer),(integer),(integer)):(LuaColor)

        --New `Color` from sRGB colorspace.
        --
        --
        --
        --# Arguments
        --
        --
        --
        --* `r` - Red channel. [0, 255]
        --
        --* `g` - Green channel. [0, 255]
        --
        --* `b` - Blue channel. [0, 255]
        --
        --* `a` - Alpha channel. [0, 255]
        --
        --
        --
        --See also [`Color::rgba`], [`Color::rgb_u8`], [`Color::hex`].

        rgba_u8 : function((integer),(integer),(integer),(integer)):(LuaColor)

        __tostring : function(LuaColor):(string)

        __add : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaColorClass

        userdata
    
    --Collection of static methods for [`LuaColor`].

        --New `Color` from sRGB colorspace.
        --
        --
        --
        --# Arguments
        --
        --
        --
        --* `r` - Red channel. [0.0, 1.0]
        --
        --* `g` - Green channel. [0.0, 1.0]
        --
        --* `b` - Blue channel. [0.0, 1.0]
        --
        --
        --
        --See also [`Color::rgba`], [`Color::rgb_u8`], [`Color::hex`].

        rgb : function((number),(number),(number)):(LuaColor)

        --New `Color` from sRGB colorspace.
        --
        --
        --
        --# Arguments
        --
        --
        --
        --* `r` - Red channel. [0.0, 1.0]
        --
        --* `g` - Green channel. [0.0, 1.0]
        --
        --* `b` - Blue channel. [0.0, 1.0]
        --
        --* `a` - Alpha channel. [0.0, 1.0]
        --
        --
        --
        --See also [`Color::rgb`], [`Color::rgba_u8`], [`Color::hex`].

        rgba : function((number),(number),(number),(number)):(LuaColor)

        --New `Color` from linear RGB colorspace.
        --
        --
        --
        --# Arguments
        --
        --
        --
        --* `r` - Red channel. [0.0, 1.0]
        --
        --* `g` - Green channel. [0.0, 1.0]
        --
        --* `b` - Blue channel. [0.0, 1.0]
        --
        --
        --
        --See also [`Color::rgb`], [`Color::rgba_linear`].

        rgb_linear : function((number),(number),(number)):(LuaColor)

        --New `Color` from linear RGB colorspace.
        --
        --
        --
        --# Arguments
        --
        --
        --
        --* `r` - Red channel. [0.0, 1.0]
        --
        --* `g` - Green channel. [0.0, 1.0]
        --
        --* `b` - Blue channel. [0.0, 1.0]
        --
        --* `a` - Alpha channel. [0.0, 1.0]
        --
        --
        --
        --See also [`Color::rgba`], [`Color::rgb_linear`].

        rgba_linear : function((number),(number),(number),(number)):(LuaColor)

        --New `Color` with HSL representation in sRGB colorspace.
        --
        --
        --
        --# Arguments
        --
        --
        --
        --* `hue` - Hue channel. [0.0, 360.0]
        --
        --* `saturation` - Saturation channel. [0.0, 1.0]
        --
        --* `lightness` - Lightness channel. [0.0, 1.0]
        --
        --
        --
        --See also [`Color::hsla`].

        hsl : function((number),(number),(number)):(LuaColor)

        --New `Color` with HSL representation in sRGB colorspace.
        --
        --
        --
        --# Arguments
        --
        --
        --
        --* `hue` - Hue channel. [0.0, 360.0]
        --
        --* `saturation` - Saturation channel. [0.0, 1.0]
        --
        --* `lightness` - Lightness channel. [0.0, 1.0]
        --
        --* `alpha` - Alpha channel. [0.0, 1.0]
        --
        --
        --
        --See also [`Color::hsl`].

        hsla : function((number),(number),(number),(number)):(LuaColor)

        --New `Color` with LCH representation in sRGB colorspace.
        --
        --
        --
        --# Arguments
        --
        --
        --
        --* `lightness` - Lightness channel. [0.0, 1.5]
        --
        --* `chroma` - Chroma channel. [0.0, 1.5]
        --
        --* `hue` - Hue channel. [0.0, 360.0]
        --
        --
        --
        --See also [`Color::lcha`].

        lch : function((number),(number),(number)):(LuaColor)

        --New `Color` with LCH representation in sRGB colorspace.
        --
        --
        --
        --# Arguments
        --
        --
        --
        --* `lightness` - Lightness channel. [0.0, 1.5]
        --
        --* `chroma` - Chroma channel. [0.0, 1.5]
        --
        --* `hue` - Hue channel. [0.0, 360.0]
        --
        --* `alpha` - Alpha channel. [0.0, 1.0]
        --
        --
        --
        --See also [`Color::lch`].

        lcha : function((number),(number),(number),(number)):(LuaColor)

        --New `Color` from sRGB colorspace.
        --
        --
        --
        --# Arguments
        --
        --
        --
        --* `r` - Red channel. [0, 255]
        --
        --* `g` - Green channel. [0, 255]
        --
        --* `b` - Blue channel. [0, 255]
        --
        --
        --
        --See also [`Color::rgb`], [`Color::rgba_u8`], [`Color::hex`].

        rgb_u8 : function((integer),(integer),(integer)):(LuaColor)

        --New `Color` from sRGB colorspace.
        --
        --
        --
        --# Arguments
        --
        --
        --
        --* `r` - Red channel. [0, 255]
        --
        --* `g` - Green channel. [0, 255]
        --
        --* `b` - Blue channel. [0, 255]
        --
        --* `a` - Alpha channel. [0, 255]
        --
        --
        --
        --See also [`Color::rgba`], [`Color::rgb_u8`], [`Color::hex`].

        rgba_u8 : function((integer),(integer),(integer),(integer)):(LuaColor)

        __add : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaAabb

        userdata
    
    --

        center : LuaVec3A

        half_extents : LuaVec3A

        min : function(LuaAabb):(LuaVec3A)

        max : function(LuaAabb):(LuaVec3A)

        from_min_max : function((LuaVec3),(LuaVec3)):(LuaAabb)

        __tostring : function(LuaAabb):(string)

    end
    
    
    record LuaAabbClass

        userdata
    
    --Collection of static methods for [`LuaAabb`].

        from_min_max : function((LuaVec3),(LuaVec3)):(LuaAabb)

    end
    
    
    record LuaCubemapFrusta

        userdata
    
    --

        __tostring : function(LuaCubemapFrusta):(string)

    end
    
    
    record LuaFrustum

        userdata
    
    --

        --Checks if an Oriented Bounding Box (obb) intersects the frustum.

        intersects_obb : function(LuaFrustum,(LuaAabb),(LuaMat4),(boolean),(boolean)):(boolean)

        --Returns a frustum derived from `view_projection`.

        from_view_projection : function(LuaMat4):(LuaFrustum)

        --Returns a frustum derived from `view_projection`,
        --
        --but with a custom far plane.

        from_view_projection_custom_far : function((LuaMat4),(LuaVec3),(LuaVec3),(number)):(LuaFrustum)

        __tostring : function(LuaFrustum):(string)

    end
    
    
    record LuaFrustumClass

        userdata
    
    --Collection of static methods for [`LuaFrustum`].

        --Returns a frustum derived from `view_projection`.

        from_view_projection : function(LuaMat4):(LuaFrustum)

        --Returns a frustum derived from `view_projection`,
        --
        --but with a custom far plane.

        from_view_projection_custom_far : function((LuaMat4),(LuaVec3),(LuaVec3),(number)):(LuaFrustum)

    end
    
    
    record LuaMsaa

        userdata
    
    --

        samples : function(LuaMsaa):(integer)

        __tostring : function(LuaMsaa):(string)

    end
    
    
    record LuaCamera

        userdata
    
    --

        -- If set, this camera will render to the given [`Viewport`] rectangle within the configured [`RenderTarget`].

        viewport : ReflectedValue

        -- Cameras with a higher order are rendered later, and thus on top of lower order cameras.

        order : integer

        -- If this is set to `true`, this camera will be rendered to its specified [`RenderTarget`]. If `false`, this
        --
        -- camera will not be rendered.

        is_active : boolean

        -- The "target" that this camera will render to.

        target : LuaRenderTarget

        -- If this is set to `true`, the camera will use an intermediate "high dynamic range" render texture.
        --
        -- This allows rendering with a wider range of lighting values.

        hdr : boolean

        -- If this is enabled, a previous camera exists that shares this camera's render target, and this camera has MSAA enabled, then the previous camera's
        --
        -- outputs will be written to the intermediate multi-sampled render target textures for this camera. This enables cameras with MSAA enabled to
        --
        -- "write their results on top" of previous camera results, and include them as a part of their render results. This is enabled by default to ensure
        --
        -- cameras with MSAA enabled layer their results in the same way as cameras without MSAA enabled by default.

        msaa_writeback : boolean

        --The projection matrix computed using this camera's [`CameraProjection`].

        projection_matrix : function(LuaCamera):(LuaMat4)

        __tostring : function(LuaCamera):(string)

    end
    
    
    record LuaRenderTarget

        userdata
    
    --

        __tostring : function(LuaRenderTarget):(string)

    end
    
    
    record LuaViewport

        userdata
    
    --

        -- The physical position to render this viewport to within the [`RenderTarget`] of this [`Camera`].
        --
        -- (0,0) corresponds to the top-left corner

        physical_position : LuaUVec2

        -- The physical size of the viewport rectangle to render to within the [`RenderTarget`] of this [`Camera`].
        --
        -- The origin of the rectangle is in the top-left corner.

        physical_size : LuaUVec2

        -- The minimum and maximum depth to render (on a scale from 0.0 to 1.0).

        depth : ReflectedValue

        __tostring : function(LuaViewport):(string)

    end
    
    
    record LuaProjection

        userdata
    
    --

        get_projection_matrix : function(LuaProjection):(LuaMat4)

        far : function(LuaProjection):(number)

        update : function(LuaProjection,(number),(number)):()

        __tostring : function(LuaProjection):(string)

    end
    
    
    record LuaOrthographicProjection

        userdata
    
    --

        -- The distance of the near clipping plane in world units.
        --
        --
        --
        -- Objects closer than this will not be rendered.
        --
        --
        --
        -- Defaults to `0.0`

        near : number

        -- The distance of the far clipping plane in world units.
        --
        --
        --
        -- Objects further than this will not be rendered.
        --
        --
        --
        -- Defaults to `1000.0`

        _far : number

        -- Specifies the origin of the viewport as a normalized position from 0 to 1, where (0, 0) is the bottom left
        --
        -- and (1, 1) is the top right. This determines where the camera's position sits inside the viewport.
        --
        --
        --
        -- When the projection scales due to viewport resizing, the position of the camera, and thereby `viewport_origin`,
        --
        -- remains at the same relative point.
        --
        --
        --
        -- Consequently, this is pivot point when scaling. With a bottom left pivot, the projection will expand
        --
        -- upwards and to the right. With a top right pivot, the projection will expand downwards and to the left.
        --
        -- Values in between will caused the projection to scale proportionally on each axis.
        --
        --
        --
        -- Defaults to `(0.5, 0.5)`, which makes scaling affect opposite sides equally, keeping the center
        --
        -- point of the viewport centered.

        viewport_origin : LuaVec2

        -- How the projection will scale when the viewport is resized.
        --
        --
        --
        -- Defaults to `ScalingMode::WindowScale(1.0)`

        scaling_mode : LuaScalingMode

        -- Scales the projection in world units.
        --
        --
        --
        -- As scale increases, the apparent size of objects decreases, and vice versa.
        --
        --
        --
        -- Defaults to `1.0`

        scale : number

        -- The area that the projection covers relative to `viewport_origin`.
        --
        --
        --
        -- Bevy's [`camera_system`](crate::camera::camera_system) automatically
        --
        -- updates this value when the viewport is resized depending on `OrthographicProjection`'s other fields.
        --
        -- In this case, `area` should not be manually modified.
        --
        --
        --
        -- It may be necessary to set this manually for shadow projections and such.

        area : LuaRect

        get_projection_matrix : function(LuaOrthographicProjection):(LuaMat4)

        far : function(LuaOrthographicProjection):(number)

        update : function(LuaOrthographicProjection,(number),(number)):()

        __tostring : function(LuaOrthographicProjection):(string)

    end
    
    
    record LuaPerspectiveProjection

        userdata
    
    --

        -- The vertical field of view (FOV) in radians.
        --
        --
        --
        -- Defaults to a value of π/4 radians or 45 degrees.

        fov : number

        -- The aspect ratio (width divided by height) of the viewing frustum.
        --
        --
        --
        -- Bevy's [`camera_system`](crate::camera::camera_system) automatically
        --
        -- updates this value when the aspect ratio of the associated window changes.
        --
        --
        --
        -- Defaults to a value of `1.0`.

        aspect_ratio : number

        -- The distance from the camera in world units of the viewing frustum's near plane.
        --
        --
        --
        -- Objects closer to the camera than this value will not be visible.
        --
        --
        --
        -- Defaults to a value of `0.1`.

        near : number

        -- The distance from the camera in world units of the viewing frustum's far plane.
        --
        --
        --
        -- Objects farther from the camera than this value will not be visible.
        --
        --
        --
        -- Defaults to a value of `1000.0`.

        _far : number

        get_projection_matrix : function(LuaPerspectiveProjection):(LuaMat4)

        far : function(LuaPerspectiveProjection):(number)

        update : function(LuaPerspectiveProjection,(number),(number)):()

        __tostring : function(LuaPerspectiveProjection):(string)

    end
    
    
    record LuaCameraRenderGraph

        userdata
    
    --

    end
    
    
    record LuaCameraRenderGraphClass

        userdata
    
    --Collection of static methods for [`LuaCameraRenderGraph`].

    end
    
    
    record LuaAssetPathId

        userdata
    
    --

        --Gets the id of the source path.

        source_path_id : function(LuaAssetPathId):(LuaSourcePathId)

        --Gets the id of the sub-asset label.

        label_id : function(LuaAssetPathId):(LuaLabelId)

        __tostring : function(LuaAssetPathId):(string)

    end
    
    
    record LuaLabelId

        userdata
    
    --

        __tostring : function(LuaLabelId):(string)

    end
    
    
    record LuaSourcePathId

        userdata
    
    --

        __tostring : function(LuaSourcePathId):(string)

    end
    
    
    record LuaHandleId

        userdata
    
    --

        __tostring : function(LuaHandleId):(string)

    end
    
    
    record LuaHandleIdClass

        userdata
    
    --Collection of static methods for [`LuaHandleId`].

    end
    
    
    record LuaVec2

        userdata
    
    --

        x : number

        y : number

        --Creates a 3D vector from `self` and the given `z` value.

        extend : function(LuaVec2,number):(LuaVec3)

        --Computes the dot product of `self` and `rhs`.

        dot : function(LuaVec2,LuaVec2):(number)

        --Returns a vector where every component is the dot product of `self` and `rhs`.

        dot_into_vec : function(LuaVec2,LuaVec2):(LuaVec2)

        --Returns a vector containing the minimum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

        min : function(LuaVec2,LuaVec2):(LuaVec2)

        --Returns a vector containing the maximum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

        max : function(LuaVec2,LuaVec2):(LuaVec2)

        --Component-wise clamping of values, similar to [`f32::clamp`].
        --
        --
        --
        --Each element in `min` must be less-or-equal to the corresponding element in `max`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `min` is greater than `max` when `glam_assert` is enabled.

        clamp : function(LuaVec2,(LuaVec2),(LuaVec2)):(LuaVec2)

        --Returns the horizontal minimum of `self`.
        --
        --
        --
        --In other words this computes `min(x, y, ..)`.

        min_element : function(LuaVec2):(number)

        --Returns the horizontal maximum of `self`.
        --
        --
        --
        --In other words this computes `max(x, y, ..)`.

        max_element : function(LuaVec2):(number)

        --Returns a vector mask containing the result of a `==` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
        --
        --elements.

        cmpeq : function(LuaVec2,LuaVec2):(LuaBVec2)

        --Returns a vector mask containing the result of a `!=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
        --
        --elements.

        cmpne : function(LuaVec2,LuaVec2):(LuaBVec2)

        --Returns a vector mask containing the result of a `>=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
        --
        --elements.

        cmpge : function(LuaVec2,LuaVec2):(LuaBVec2)

        --Returns a vector mask containing the result of a `>` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
        --
        --elements.

        cmpgt : function(LuaVec2,LuaVec2):(LuaBVec2)

        --Returns a vector mask containing the result of a `<=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
        --
        --elements.

        cmple : function(LuaVec2,LuaVec2):(LuaBVec2)

        --Returns a vector mask containing the result of a `<` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
        --
        --elements.

        cmplt : function(LuaVec2,LuaVec2):(LuaBVec2)

        --Returns a vector containing the absolute value of each element of `self`.

        abs : function(LuaVec2):(LuaVec2)

        --Returns a vector with elements representing the sign of `self`.
        --
        --
        --
        --- `1.0` if the number is positive, `+0.0` or `INFINITY`
        --
        --- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
        --
        --- `NAN` if the number is `NAN`

        signum : function(LuaVec2):(LuaVec2)

        --Returns a vector with signs of `rhs` and the magnitudes of `self`.

        copysign : function(LuaVec2,LuaVec2):(LuaVec2)

        --Returns a bitmask with the lowest 2 bits set to the sign bits from the elements of `self`.
        --
        --
        --
        --A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
        --
        --into the first lowest bit, element `y` into the second, etc.

        is_negative_bitmask : function(LuaVec2):(integer)

        --Returns `true` if, and only if, all elements are finite.  If any element is either
        --
        --`NaN`, positive or negative infinity, this will return `false`.

        is_finite : function(LuaVec2):(boolean)

        --Returns `true` if any elements are `NaN`.

        is_nan : function(LuaVec2):(boolean)

        --Performs `is_nan` on each element of self, returning a vector mask of the results.
        --
        --
        --
        --In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.

        is_nan_mask : function(LuaVec2):(LuaBVec2)

        --Computes the length of `self`.

        length : function(LuaVec2):(number)

        --Computes the squared length of `self`.
        --
        --
        --
        --This is faster than `length()` as it avoids a square root operation.

        length_squared : function(LuaVec2):(number)

        --Computes `1.0 / length()`.
        --
        --
        --
        --For valid results, `self` must _not_ be of length zero.

        length_recip : function(LuaVec2):(number)

        --Computes the Euclidean distance between two points in space.

        distance : function(LuaVec2,LuaVec2):(number)

        --Compute the squared euclidean distance between two points in space.

        distance_squared : function(LuaVec2,LuaVec2):(number)

        --Returns `self` normalized to length 1.0.
        --
        --
        --
        --For valid results, `self` must _not_ be of length zero, nor very close to zero.
        --
        --
        --
        --See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
        --
        --
        --
        --Panics
        --
        --
        --
        --Will panic if `self` is zero length when `glam_assert` is enabled.

        normalize : function(LuaVec2):(LuaVec2)

        --Returns `self` normalized to length 1.0 if possible, else returns zero.
        --
        --
        --
        --In particular, if the input is zero (or very close to zero), or non-finite,
        --
        --the result of this operation will be zero.
        --
        --
        --
        --See also [`Self::try_normalize()`].

        normalize_or_zero : function(LuaVec2):(LuaVec2)

        --Returns whether `self` is length `1.0` or not.
        --
        --
        --
        --Uses a precision threshold of `1e-6`.

        is_normalized : function(LuaVec2):(boolean)

        --Returns the vector projection of `self` onto `rhs`.
        --
        --
        --
        --`rhs` must be of non-zero length.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is zero length when `glam_assert` is enabled.

        project_onto : function(LuaVec2,LuaVec2):(LuaVec2)

        --Returns the vector rejection of `self` from `rhs`.
        --
        --
        --
        --The vector rejection is the vector perpendicular to the projection of `self` onto
        --
        --`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
        --
        --
        --
        --`rhs` must be of non-zero length.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` has a length of zero when `glam_assert` is enabled.

        reject_from : function(LuaVec2,LuaVec2):(LuaVec2)

        --Returns the vector projection of `self` onto `rhs`.
        --
        --
        --
        --`rhs` must be normalized.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is not normalized when `glam_assert` is enabled.

        project_onto_normalized : function(LuaVec2,LuaVec2):(LuaVec2)

        --Returns the vector rejection of `self` from `rhs`.
        --
        --
        --
        --The vector rejection is the vector perpendicular to the projection of `self` onto
        --
        --`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
        --
        --
        --
        --`rhs` must be normalized.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is not normalized when `glam_assert` is enabled.

        reject_from_normalized : function(LuaVec2,LuaVec2):(LuaVec2)

        --Returns a vector containing the nearest integer to a number for each element of `self`.
        --
        --Round half-way cases away from 0.0.

        round : function(LuaVec2):(LuaVec2)

        --Returns a vector containing the largest integer less than or equal to a number for each
        --
        --element of `self`.

        floor : function(LuaVec2):(LuaVec2)

        --Returns a vector containing the smallest integer greater than or equal to a number for
        --
        --each element of `self`.

        ceil : function(LuaVec2):(LuaVec2)

        --Returns a vector containing the integer part each element of `self`. This means numbers are
        --
        --always truncated towards zero.

        trunc : function(LuaVec2):(LuaVec2)

        --Returns a vector containing the fractional part of the vector, e.g. `self -
        --
        --self.floor()`.
        --
        --
        --
        --Note that this is fast but not precise for large numbers.

        fract : function(LuaVec2):(LuaVec2)

        --Returns a vector containing `e^self` (the exponential function) for each element of
        --
        --`self`.

        exp : function(LuaVec2):(LuaVec2)

        --Returns a vector containing each element of `self` raised to the power of `n`.

        powf : function(LuaVec2,number):(LuaVec2)

        --Returns a vector containing the reciprocal `1.0/n` of each element of `self`.

        recip : function(LuaVec2):(LuaVec2)

        --Performs a linear interpolation between `self` and `rhs` based on the value `s`.
        --
        --
        --
        --When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
        --
        --will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
        --
        --extrapolated.

        lerp : function(LuaVec2,(LuaVec2),(number)):(LuaVec2)

        --Returns true if the absolute difference of all elements between `self` and `rhs` is
        --
        --less than or equal to `max_abs_diff`.
        --
        --
        --
        --This can be used to compare if two vectors contain similar elements. It works best when
        --
        --comparing with a known value. The `max_abs_diff` that should be used used depends on
        --
        --the values being compared against.
        --
        --
        --
        --For more see
        --
        --[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

        abs_diff_eq : function(LuaVec2,(LuaVec2),(number)):(boolean)

        --Returns a vector with a length no less than `min` and no more than `max`
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `min` is greater than `max` when `glam_assert` is enabled.

        clamp_length : function(LuaVec2,(number),(number)):(LuaVec2)

        --Returns a vector with a length no more than `max`

        clamp_length_max : function(LuaVec2,number):(LuaVec2)

        --Returns a vector with a length no less than `min`

        clamp_length_min : function(LuaVec2,number):(LuaVec2)

        --Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
        --
        --error, yielding a more accurate result than an unfused multiply-add.
        --
        --
        --
        --Using `mul_add` *may* be more performant than an unfused multiply-add if the target
        --
        --architecture has a dedicated fma CPU instruction. However, this is not always true,
        --
        --and will be heavily dependant on designing algorithms with specific target hardware in
        --
        --mind.

        mul_add : function(LuaVec2,(LuaVec2),(LuaVec2)):(LuaVec2)

        --Returns the angle (in radians) between `self` and `rhs` in the range `[-π, +π]`.
        --
        --
        --
        --The inputs do not need to be unit vectors however they must be non-zero.

        angle_between : function(LuaVec2,LuaVec2):(number)

        --Returns a vector that is equal to `self` rotated by 90 degrees.

        perp : function(LuaVec2):(LuaVec2)

        --The perpendicular dot product of `self` and `rhs`.
        --
        --Also known as the wedge product, 2D cross product, and determinant.

        perp_dot : function(LuaVec2,LuaVec2):(number)

        --Returns `rhs` rotated by the angle of `self`. If `self` is normalized,
        --
        --then this just rotation. This is what you usually want. Otherwise,
        --
        --it will be like a rotation with a multiplication by `self`'s length.

        rotate : function(LuaVec2,LuaVec2):(LuaVec2)

        --Casts all elements of `self` to `f64`.

        as_dvec2 : function(LuaVec2):(LuaDVec2)

        --Casts all elements of `self` to `i32`.

        as_ivec2 : function(LuaVec2):(LuaIVec2)

        --Casts all elements of `self` to `u32`.

        as_uvec2 : function(LuaVec2):(LuaUVec2)

        --Creates a new vector.

        new : function((number),(number)):(LuaVec2)

        --Creates a vector with all elements set to `v`.

        splat : function(number):(LuaVec2)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec2),(LuaVec2),(LuaVec2)):(LuaVec2)

        --Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in
        --
        --conjunction with the [`rotate()`][Self::rotate()] method, e.g.
        --
        --`Vec2::from_angle(PI).rotate(Vec2::Y)` will create the vector `[-1, 0]`
        --
        --and rotate [`Vec2::Y`] around it returning `-Vec2::Y`.

        from_angle : function(number):(LuaVec2)

        __tostring : function(LuaVec2):(string)

        __unm : function(LuaVec2):(LuaVec2)

        __index : function(LuaVec2,integer):(number)

        __newindex : function(LuaVec2,(integer),(number)):()

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaVec2Class

        userdata
    
    --Collection of static methods for [`LuaVec2`].

        --Creates a new vector.

        new : function((number),(number)):(LuaVec2)

        --Creates a vector with all elements set to `v`.

        splat : function(number):(LuaVec2)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec2),(LuaVec2),(LuaVec2)):(LuaVec2)

        --Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in
        --
        --conjunction with the [`rotate()`][Self::rotate()] method, e.g.
        --
        --`Vec2::from_angle(PI).rotate(Vec2::Y)` will create the vector `[-1, 0]`
        --
        --and rotate [`Vec2::Y`] around it returning `-Vec2::Y`.

        from_angle : function(number):(LuaVec2)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaVec3

        userdata
    
    --

        x : number

        y : number

        z : number

        --Creates a 4D vector from `self` and the given `w` value.

        extend : function(LuaVec3,number):(LuaVec4)

        --Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
        --
        --
        --
        --Truncation may also be performed by using [`self.xy()`][crate::swizzles::Vec3Swizzles::xy()].

        truncate : function(LuaVec3):(LuaVec2)

        --Computes the dot product of `self` and `rhs`.

        dot : function(LuaVec3,LuaVec3):(number)

        --Returns a vector where every component is the dot product of `self` and `rhs`.

        dot_into_vec : function(LuaVec3,LuaVec3):(LuaVec3)

        --Computes the cross product of `self` and `rhs`.

        cross : function(LuaVec3,LuaVec3):(LuaVec3)

        --Returns a vector containing the minimum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

        min : function(LuaVec3,LuaVec3):(LuaVec3)

        --Returns a vector containing the maximum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

        max : function(LuaVec3,LuaVec3):(LuaVec3)

        --Component-wise clamping of values, similar to [`f32::clamp`].
        --
        --
        --
        --Each element in `min` must be less-or-equal to the corresponding element in `max`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `min` is greater than `max` when `glam_assert` is enabled.

        clamp : function(LuaVec3,(LuaVec3),(LuaVec3)):(LuaVec3)

        --Returns the horizontal minimum of `self`.
        --
        --
        --
        --In other words this computes `min(x, y, ..)`.

        min_element : function(LuaVec3):(number)

        --Returns the horizontal maximum of `self`.
        --
        --
        --
        --In other words this computes `max(x, y, ..)`.

        max_element : function(LuaVec3):(number)

        --Returns a vector mask containing the result of a `==` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
        --
        --elements.

        cmpeq : function(LuaVec3,LuaVec3):(LuaBVec3)

        --Returns a vector mask containing the result of a `!=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
        --
        --elements.

        cmpne : function(LuaVec3,LuaVec3):(LuaBVec3)

        --Returns a vector mask containing the result of a `>=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
        --
        --elements.

        cmpge : function(LuaVec3,LuaVec3):(LuaBVec3)

        --Returns a vector mask containing the result of a `>` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
        --
        --elements.

        cmpgt : function(LuaVec3,LuaVec3):(LuaBVec3)

        --Returns a vector mask containing the result of a `<=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
        --
        --elements.

        cmple : function(LuaVec3,LuaVec3):(LuaBVec3)

        --Returns a vector mask containing the result of a `<` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
        --
        --elements.

        cmplt : function(LuaVec3,LuaVec3):(LuaBVec3)

        --Returns a vector containing the absolute value of each element of `self`.

        abs : function(LuaVec3):(LuaVec3)

        --Returns a vector with elements representing the sign of `self`.
        --
        --
        --
        --- `1.0` if the number is positive, `+0.0` or `INFINITY`
        --
        --- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
        --
        --- `NAN` if the number is `NAN`

        signum : function(LuaVec3):(LuaVec3)

        --Returns a vector with signs of `rhs` and the magnitudes of `self`.

        copysign : function(LuaVec3,LuaVec3):(LuaVec3)

        --Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of `self`.
        --
        --
        --
        --A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
        --
        --into the first lowest bit, element `y` into the second, etc.

        is_negative_bitmask : function(LuaVec3):(integer)

        --Returns `true` if, and only if, all elements are finite.  If any element is either
        --
        --`NaN`, positive or negative infinity, this will return `false`.

        is_finite : function(LuaVec3):(boolean)

        --Returns `true` if any elements are `NaN`.

        is_nan : function(LuaVec3):(boolean)

        --Performs `is_nan` on each element of self, returning a vector mask of the results.
        --
        --
        --
        --In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.

        is_nan_mask : function(LuaVec3):(LuaBVec3)

        --Computes the length of `self`.

        length : function(LuaVec3):(number)

        --Computes the squared length of `self`.
        --
        --
        --
        --This is faster than `length()` as it avoids a square root operation.

        length_squared : function(LuaVec3):(number)

        --Computes `1.0 / length()`.
        --
        --
        --
        --For valid results, `self` must _not_ be of length zero.

        length_recip : function(LuaVec3):(number)

        --Computes the Euclidean distance between two points in space.

        distance : function(LuaVec3,LuaVec3):(number)

        --Compute the squared euclidean distance between two points in space.

        distance_squared : function(LuaVec3,LuaVec3):(number)

        --Returns `self` normalized to length 1.0.
        --
        --
        --
        --For valid results, `self` must _not_ be of length zero, nor very close to zero.
        --
        --
        --
        --See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
        --
        --
        --
        --Panics
        --
        --
        --
        --Will panic if `self` is zero length when `glam_assert` is enabled.

        normalize : function(LuaVec3):(LuaVec3)

        --Returns `self` normalized to length 1.0 if possible, else returns zero.
        --
        --
        --
        --In particular, if the input is zero (or very close to zero), or non-finite,
        --
        --the result of this operation will be zero.
        --
        --
        --
        --See also [`Self::try_normalize()`].

        normalize_or_zero : function(LuaVec3):(LuaVec3)

        --Returns whether `self` is length `1.0` or not.
        --
        --
        --
        --Uses a precision threshold of `1e-6`.

        is_normalized : function(LuaVec3):(boolean)

        --Returns the vector projection of `self` onto `rhs`.
        --
        --
        --
        --`rhs` must be of non-zero length.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is zero length when `glam_assert` is enabled.

        project_onto : function(LuaVec3,LuaVec3):(LuaVec3)

        --Returns the vector rejection of `self` from `rhs`.
        --
        --
        --
        --The vector rejection is the vector perpendicular to the projection of `self` onto
        --
        --`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
        --
        --
        --
        --`rhs` must be of non-zero length.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` has a length of zero when `glam_assert` is enabled.

        reject_from : function(LuaVec3,LuaVec3):(LuaVec3)

        --Returns the vector projection of `self` onto `rhs`.
        --
        --
        --
        --`rhs` must be normalized.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is not normalized when `glam_assert` is enabled.

        project_onto_normalized : function(LuaVec3,LuaVec3):(LuaVec3)

        --Returns the vector rejection of `self` from `rhs`.
        --
        --
        --
        --The vector rejection is the vector perpendicular to the projection of `self` onto
        --
        --`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
        --
        --
        --
        --`rhs` must be normalized.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is not normalized when `glam_assert` is enabled.

        reject_from_normalized : function(LuaVec3,LuaVec3):(LuaVec3)

        --Returns a vector containing the nearest integer to a number for each element of `self`.
        --
        --Round half-way cases away from 0.0.

        round : function(LuaVec3):(LuaVec3)

        --Returns a vector containing the largest integer less than or equal to a number for each
        --
        --element of `self`.

        floor : function(LuaVec3):(LuaVec3)

        --Returns a vector containing the smallest integer greater than or equal to a number for
        --
        --each element of `self`.

        ceil : function(LuaVec3):(LuaVec3)

        --Returns a vector containing the integer part each element of `self`. This means numbers are
        --
        --always truncated towards zero.

        trunc : function(LuaVec3):(LuaVec3)

        --Returns a vector containing the fractional part of the vector, e.g. `self -
        --
        --self.floor()`.
        --
        --
        --
        --Note that this is fast but not precise for large numbers.

        fract : function(LuaVec3):(LuaVec3)

        --Returns a vector containing `e^self` (the exponential function) for each element of
        --
        --`self`.

        exp : function(LuaVec3):(LuaVec3)

        --Returns a vector containing each element of `self` raised to the power of `n`.

        powf : function(LuaVec3,number):(LuaVec3)

        --Returns a vector containing the reciprocal `1.0/n` of each element of `self`.

        recip : function(LuaVec3):(LuaVec3)

        --Performs a linear interpolation between `self` and `rhs` based on the value `s`.
        --
        --
        --
        --When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
        --
        --will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
        --
        --extrapolated.

        lerp : function(LuaVec3,(LuaVec3),(number)):(LuaVec3)

        --Returns true if the absolute difference of all elements between `self` and `rhs` is
        --
        --less than or equal to `max_abs_diff`.
        --
        --
        --
        --This can be used to compare if two vectors contain similar elements. It works best when
        --
        --comparing with a known value. The `max_abs_diff` that should be used used depends on
        --
        --the values being compared against.
        --
        --
        --
        --For more see
        --
        --[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

        abs_diff_eq : function(LuaVec3,(LuaVec3),(number)):(boolean)

        --Returns a vector with a length no less than `min` and no more than `max`
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `min` is greater than `max` when `glam_assert` is enabled.

        clamp_length : function(LuaVec3,(number),(number)):(LuaVec3)

        --Returns a vector with a length no more than `max`

        clamp_length_max : function(LuaVec3,number):(LuaVec3)

        --Returns a vector with a length no less than `min`

        clamp_length_min : function(LuaVec3,number):(LuaVec3)

        --Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
        --
        --error, yielding a more accurate result than an unfused multiply-add.
        --
        --
        --
        --Using `mul_add` *may* be more performant than an unfused multiply-add if the target
        --
        --architecture has a dedicated fma CPU instruction. However, this is not always true,
        --
        --and will be heavily dependant on designing algorithms with specific target hardware in
        --
        --mind.

        mul_add : function(LuaVec3,(LuaVec3),(LuaVec3)):(LuaVec3)

        --Returns the angle (in radians) between two vectors.
        --
        --
        --
        --The inputs do not need to be unit vectors however they must be non-zero.

        angle_between : function(LuaVec3,LuaVec3):(number)

        --Returns some vector that is orthogonal to the given one.
        --
        --
        --
        --The input vector must be finite and non-zero.
        --
        --
        --
        --The output vector is not necessarily unit length. For that use
        --
        --[`Self::any_orthonormal_vector()`] instead.

        any_orthogonal_vector : function(LuaVec3):(LuaVec3)

        --Returns any unit vector that is orthogonal to the given one.
        --
        --
        --
        --The input vector must be unit length.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `self` is not normalized when `glam_assert` is enabled.

        any_orthonormal_vector : function(LuaVec3):(LuaVec3)

        --Casts all elements of `self` to `f64`.

        as_dvec3 : function(LuaVec3):(LuaDVec3)

        --Casts all elements of `self` to `i32`.

        as_ivec3 : function(LuaVec3):(LuaIVec3)

        --Casts all elements of `self` to `u32`.

        as_uvec3 : function(LuaVec3):(LuaUVec3)

        --Creates a new vector.

        new : function((number),(number),(number)):(LuaVec3)

        --Creates a vector with all elements set to `v`.

        splat : function(number):(LuaVec3)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec3),(LuaVec3),(LuaVec3)):(LuaVec3)

        __tostring : function(LuaVec3):(string)

        __unm : function(LuaVec3):(LuaVec3)

        __index : function(LuaVec3,integer):(number)

        __newindex : function(LuaVec3,(integer),(number)):()

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaVec3Class

        userdata
    
    --Collection of static methods for [`LuaVec3`].

        --Creates a new vector.

        new : function((number),(number),(number)):(LuaVec3)

        --Creates a vector with all elements set to `v`.

        splat : function(number):(LuaVec3)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec3),(LuaVec3),(LuaVec3)):(LuaVec3)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaVec3A

        userdata
    
    --

        --Creates a 4D vector from `self` and the given `w` value.

        extend : function(LuaVec3A,number):(LuaVec4)

        --Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
        --
        --
        --
        --Truncation may also be performed by using [`self.xy()`][crate::swizzles::Vec3Swizzles::xy()].

        truncate : function(LuaVec3A):(LuaVec2)

        --Computes the dot product of `self` and `rhs`.

        dot : function(LuaVec3A,LuaVec3A):(number)

        --Returns a vector where every component is the dot product of `self` and `rhs`.

        dot_into_vec : function(LuaVec3A,LuaVec3A):(LuaVec3A)

        --Computes the cross product of `self` and `rhs`.

        cross : function(LuaVec3A,LuaVec3A):(LuaVec3A)

        --Returns a vector containing the minimum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

        min : function(LuaVec3A,LuaVec3A):(LuaVec3A)

        --Returns a vector containing the maximum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

        max : function(LuaVec3A,LuaVec3A):(LuaVec3A)

        --Component-wise clamping of values, similar to [`f32::clamp`].
        --
        --
        --
        --Each element in `min` must be less-or-equal to the corresponding element in `max`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `min` is greater than `max` when `glam_assert` is enabled.

        clamp : function(LuaVec3A,(LuaVec3A),(LuaVec3A)):(LuaVec3A)

        --Returns the horizontal minimum of `self`.
        --
        --
        --
        --In other words this computes `min(x, y, ..)`.

        min_element : function(LuaVec3A):(number)

        --Returns the horizontal maximum of `self`.
        --
        --
        --
        --In other words this computes `max(x, y, ..)`.

        max_element : function(LuaVec3A):(number)

        --Returns a vector mask containing the result of a `==` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
        --
        --elements.

        cmpeq : function(LuaVec3A,LuaVec3A):(LuaBVec3A)

        --Returns a vector mask containing the result of a `!=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
        --
        --elements.

        cmpne : function(LuaVec3A,LuaVec3A):(LuaBVec3A)

        --Returns a vector mask containing the result of a `>=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
        --
        --elements.

        cmpge : function(LuaVec3A,LuaVec3A):(LuaBVec3A)

        --Returns a vector mask containing the result of a `>` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
        --
        --elements.

        cmpgt : function(LuaVec3A,LuaVec3A):(LuaBVec3A)

        --Returns a vector mask containing the result of a `<=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
        --
        --elements.

        cmple : function(LuaVec3A,LuaVec3A):(LuaBVec3A)

        --Returns a vector mask containing the result of a `<` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
        --
        --elements.

        cmplt : function(LuaVec3A,LuaVec3A):(LuaBVec3A)

        --Returns a vector containing the absolute value of each element of `self`.

        abs : function(LuaVec3A):(LuaVec3A)

        --Returns a vector with elements representing the sign of `self`.
        --
        --
        --
        --- `1.0` if the number is positive, `+0.0` or `INFINITY`
        --
        --- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
        --
        --- `NAN` if the number is `NAN`

        signum : function(LuaVec3A):(LuaVec3A)

        --Returns a vector with signs of `rhs` and the magnitudes of `self`.

        copysign : function(LuaVec3A,LuaVec3A):(LuaVec3A)

        --Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of `self`.
        --
        --
        --
        --A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
        --
        --into the first lowest bit, element `y` into the second, etc.

        is_negative_bitmask : function(LuaVec3A):(integer)

        --Returns `true` if, and only if, all elements are finite.  If any element is either
        --
        --`NaN`, positive or negative infinity, this will return `false`.

        is_finite : function(LuaVec3A):(boolean)

        --Returns `true` if any elements are `NaN`.

        is_nan : function(LuaVec3A):(boolean)

        --Performs `is_nan` on each element of self, returning a vector mask of the results.
        --
        --
        --
        --In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.

        is_nan_mask : function(LuaVec3A):(LuaBVec3A)

        --Computes the length of `self`.

        length : function(LuaVec3A):(number)

        --Computes the squared length of `self`.
        --
        --
        --
        --This is faster than `length()` as it avoids a square root operation.

        length_squared : function(LuaVec3A):(number)

        --Computes `1.0 / length()`.
        --
        --
        --
        --For valid results, `self` must _not_ be of length zero.

        length_recip : function(LuaVec3A):(number)

        --Computes the Euclidean distance between two points in space.

        distance : function(LuaVec3A,LuaVec3A):(number)

        --Compute the squared euclidean distance between two points in space.

        distance_squared : function(LuaVec3A,LuaVec3A):(number)

        --Returns `self` normalized to length 1.0.
        --
        --
        --
        --For valid results, `self` must _not_ be of length zero, nor very close to zero.
        --
        --
        --
        --See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
        --
        --
        --
        --Panics
        --
        --
        --
        --Will panic if `self` is zero length when `glam_assert` is enabled.

        normalize : function(LuaVec3A):(LuaVec3A)

        --Returns `self` normalized to length 1.0 if possible, else returns zero.
        --
        --
        --
        --In particular, if the input is zero (or very close to zero), or non-finite,
        --
        --the result of this operation will be zero.
        --
        --
        --
        --See also [`Self::try_normalize()`].

        normalize_or_zero : function(LuaVec3A):(LuaVec3A)

        --Returns whether `self` is length `1.0` or not.
        --
        --
        --
        --Uses a precision threshold of `1e-6`.

        is_normalized : function(LuaVec3A):(boolean)

        --Returns the vector projection of `self` onto `rhs`.
        --
        --
        --
        --`rhs` must be of non-zero length.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is zero length when `glam_assert` is enabled.

        project_onto : function(LuaVec3A,LuaVec3A):(LuaVec3A)

        --Returns the vector rejection of `self` from `rhs`.
        --
        --
        --
        --The vector rejection is the vector perpendicular to the projection of `self` onto
        --
        --`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
        --
        --
        --
        --`rhs` must be of non-zero length.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` has a length of zero when `glam_assert` is enabled.

        reject_from : function(LuaVec3A,LuaVec3A):(LuaVec3A)

        --Returns the vector projection of `self` onto `rhs`.
        --
        --
        --
        --`rhs` must be normalized.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is not normalized when `glam_assert` is enabled.

        project_onto_normalized : function(LuaVec3A,LuaVec3A):(LuaVec3A)

        --Returns the vector rejection of `self` from `rhs`.
        --
        --
        --
        --The vector rejection is the vector perpendicular to the projection of `self` onto
        --
        --`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
        --
        --
        --
        --`rhs` must be normalized.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is not normalized when `glam_assert` is enabled.

        reject_from_normalized : function(LuaVec3A,LuaVec3A):(LuaVec3A)

        --Returns a vector containing the nearest integer to a number for each element of `self`.
        --
        --Round half-way cases away from 0.0.

        round : function(LuaVec3A):(LuaVec3A)

        --Returns a vector containing the largest integer less than or equal to a number for each
        --
        --element of `self`.

        floor : function(LuaVec3A):(LuaVec3A)

        --Returns a vector containing the smallest integer greater than or equal to a number for
        --
        --each element of `self`.

        ceil : function(LuaVec3A):(LuaVec3A)

        --Returns a vector containing the integer part each element of `self`. This means numbers are
        --
        --always truncated towards zero.

        trunc : function(LuaVec3A):(LuaVec3A)

        --Returns a vector containing the fractional part of the vector, e.g. `self -
        --
        --self.floor()`.
        --
        --
        --
        --Note that this is fast but not precise for large numbers.

        fract : function(LuaVec3A):(LuaVec3A)

        --Returns a vector containing `e^self` (the exponential function) for each element of
        --
        --`self`.

        exp : function(LuaVec3A):(LuaVec3A)

        --Returns a vector containing each element of `self` raised to the power of `n`.

        powf : function(LuaVec3A,number):(LuaVec3A)

        --Returns a vector containing the reciprocal `1.0/n` of each element of `self`.

        recip : function(LuaVec3A):(LuaVec3A)

        --Performs a linear interpolation between `self` and `rhs` based on the value `s`.
        --
        --
        --
        --When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
        --
        --will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
        --
        --extrapolated.

        lerp : function(LuaVec3A,(LuaVec3A),(number)):(LuaVec3A)

        --Returns true if the absolute difference of all elements between `self` and `rhs` is
        --
        --less than or equal to `max_abs_diff`.
        --
        --
        --
        --This can be used to compare if two vectors contain similar elements. It works best when
        --
        --comparing with a known value. The `max_abs_diff` that should be used used depends on
        --
        --the values being compared against.
        --
        --
        --
        --For more see
        --
        --[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

        abs_diff_eq : function(LuaVec3A,(LuaVec3A),(number)):(boolean)

        --Returns a vector with a length no less than `min` and no more than `max`
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `min` is greater than `max` when `glam_assert` is enabled.

        clamp_length : function(LuaVec3A,(number),(number)):(LuaVec3A)

        --Returns a vector with a length no more than `max`

        clamp_length_max : function(LuaVec3A,number):(LuaVec3A)

        --Returns a vector with a length no less than `min`

        clamp_length_min : function(LuaVec3A,number):(LuaVec3A)

        --Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
        --
        --error, yielding a more accurate result than an unfused multiply-add.
        --
        --
        --
        --Using `mul_add` *may* be more performant than an unfused multiply-add if the target
        --
        --architecture has a dedicated fma CPU instruction. However, this is not always true,
        --
        --and will be heavily dependant on designing algorithms with specific target hardware in
        --
        --mind.

        mul_add : function(LuaVec3A,(LuaVec3A),(LuaVec3A)):(LuaVec3A)

        --Returns the angle (in radians) between two vectors.
        --
        --
        --
        --The inputs do not need to be unit vectors however they must be non-zero.

        angle_between : function(LuaVec3A,LuaVec3A):(number)

        --Returns some vector that is orthogonal to the given one.
        --
        --
        --
        --The input vector must be finite and non-zero.
        --
        --
        --
        --The output vector is not necessarily unit length. For that use
        --
        --[`Self::any_orthonormal_vector()`] instead.

        any_orthogonal_vector : function(LuaVec3A):(LuaVec3A)

        --Returns any unit vector that is orthogonal to the given one.
        --
        --
        --
        --The input vector must be unit length.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `self` is not normalized when `glam_assert` is enabled.

        any_orthonormal_vector : function(LuaVec3A):(LuaVec3A)

        --Casts all elements of `self` to `f64`.

        as_dvec3 : function(LuaVec3A):(LuaDVec3)

        --Casts all elements of `self` to `i32`.

        as_ivec3 : function(LuaVec3A):(LuaIVec3)

        --Casts all elements of `self` to `u32`.

        as_uvec3 : function(LuaVec3A):(LuaUVec3)

        --Creates a new vector.

        new : function((number),(number),(number)):(LuaVec3A)

        --Creates a vector with all elements set to `v`.

        splat : function(number):(LuaVec3A)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec3A),(LuaVec3A),(LuaVec3A)):(LuaVec3A)

        __tostring : function(LuaVec3A):(string)

        __unm : function(LuaVec3A):(LuaVec3A)

        __index : function(LuaVec3A,integer):(number)

        __newindex : function(LuaVec3A,(integer),(number)):()

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaVec3AClass

        userdata
    
    --Collection of static methods for [`LuaVec3A`].

        --Creates a new vector.

        new : function((number),(number),(number)):(LuaVec3A)

        --Creates a vector with all elements set to `v`.

        splat : function(number):(LuaVec3A)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec3A),(LuaVec3A),(LuaVec3A)):(LuaVec3A)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaVec4

        userdata
    
    --

        --Creates a 2D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
        --
        --
        --
        --Truncation to [`Vec3`] may also be performed by using [`self.xyz()`][crate::swizzles::Vec4Swizzles::xyz()].
        --
        --
        --
        --To truncate to [`Vec3A`] use [`Vec3A::from()`].

        truncate : function(LuaVec4):(LuaVec3)

        --Computes the dot product of `self` and `rhs`.

        dot : function(LuaVec4,LuaVec4):(number)

        --Returns a vector where every component is the dot product of `self` and `rhs`.

        dot_into_vec : function(LuaVec4,LuaVec4):(LuaVec4)

        --Returns a vector containing the minimum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

        min : function(LuaVec4,LuaVec4):(LuaVec4)

        --Returns a vector containing the maximum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

        max : function(LuaVec4,LuaVec4):(LuaVec4)

        --Component-wise clamping of values, similar to [`f32::clamp`].
        --
        --
        --
        --Each element in `min` must be less-or-equal to the corresponding element in `max`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `min` is greater than `max` when `glam_assert` is enabled.

        clamp : function(LuaVec4,(LuaVec4),(LuaVec4)):(LuaVec4)

        --Returns the horizontal minimum of `self`.
        --
        --
        --
        --In other words this computes `min(x, y, ..)`.

        min_element : function(LuaVec4):(number)

        --Returns the horizontal maximum of `self`.
        --
        --
        --
        --In other words this computes `max(x, y, ..)`.

        max_element : function(LuaVec4):(number)

        --Returns a vector mask containing the result of a `==` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
        --
        --elements.

        cmpeq : function(LuaVec4,LuaVec4):(LuaBVec4A)

        --Returns a vector mask containing the result of a `!=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
        --
        --elements.

        cmpne : function(LuaVec4,LuaVec4):(LuaBVec4A)

        --Returns a vector mask containing the result of a `>=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
        --
        --elements.

        cmpge : function(LuaVec4,LuaVec4):(LuaBVec4A)

        --Returns a vector mask containing the result of a `>` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
        --
        --elements.

        cmpgt : function(LuaVec4,LuaVec4):(LuaBVec4A)

        --Returns a vector mask containing the result of a `<=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
        --
        --elements.

        cmple : function(LuaVec4,LuaVec4):(LuaBVec4A)

        --Returns a vector mask containing the result of a `<` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
        --
        --elements.

        cmplt : function(LuaVec4,LuaVec4):(LuaBVec4A)

        --Returns a vector containing the absolute value of each element of `self`.

        abs : function(LuaVec4):(LuaVec4)

        --Returns a vector with elements representing the sign of `self`.
        --
        --
        --
        --- `1.0` if the number is positive, `+0.0` or `INFINITY`
        --
        --- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
        --
        --- `NAN` if the number is `NAN`

        signum : function(LuaVec4):(LuaVec4)

        --Returns a vector with signs of `rhs` and the magnitudes of `self`.

        copysign : function(LuaVec4,LuaVec4):(LuaVec4)

        --Returns a bitmask with the lowest 4 bits set to the sign bits from the elements of `self`.
        --
        --
        --
        --A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
        --
        --into the first lowest bit, element `y` into the second, etc.

        is_negative_bitmask : function(LuaVec4):(integer)

        --Returns `true` if, and only if, all elements are finite.  If any element is either
        --
        --`NaN`, positive or negative infinity, this will return `false`.

        is_finite : function(LuaVec4):(boolean)

        --Returns `true` if any elements are `NaN`.

        is_nan : function(LuaVec4):(boolean)

        --Performs `is_nan` on each element of self, returning a vector mask of the results.
        --
        --
        --
        --In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.

        is_nan_mask : function(LuaVec4):(LuaBVec4A)

        --Computes the length of `self`.

        length : function(LuaVec4):(number)

        --Computes the squared length of `self`.
        --
        --
        --
        --This is faster than `length()` as it avoids a square root operation.

        length_squared : function(LuaVec4):(number)

        --Computes `1.0 / length()`.
        --
        --
        --
        --For valid results, `self` must _not_ be of length zero.

        length_recip : function(LuaVec4):(number)

        --Computes the Euclidean distance between two points in space.

        distance : function(LuaVec4,LuaVec4):(number)

        --Compute the squared euclidean distance between two points in space.

        distance_squared : function(LuaVec4,LuaVec4):(number)

        --Returns `self` normalized to length 1.0.
        --
        --
        --
        --For valid results, `self` must _not_ be of length zero, nor very close to zero.
        --
        --
        --
        --See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
        --
        --
        --
        --Panics
        --
        --
        --
        --Will panic if `self` is zero length when `glam_assert` is enabled.

        normalize : function(LuaVec4):(LuaVec4)

        --Returns `self` normalized to length 1.0 if possible, else returns zero.
        --
        --
        --
        --In particular, if the input is zero (or very close to zero), or non-finite,
        --
        --the result of this operation will be zero.
        --
        --
        --
        --See also [`Self::try_normalize()`].

        normalize_or_zero : function(LuaVec4):(LuaVec4)

        --Returns whether `self` is length `1.0` or not.
        --
        --
        --
        --Uses a precision threshold of `1e-6`.

        is_normalized : function(LuaVec4):(boolean)

        --Returns the vector projection of `self` onto `rhs`.
        --
        --
        --
        --`rhs` must be of non-zero length.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is zero length when `glam_assert` is enabled.

        project_onto : function(LuaVec4,LuaVec4):(LuaVec4)

        --Returns the vector rejection of `self` from `rhs`.
        --
        --
        --
        --The vector rejection is the vector perpendicular to the projection of `self` onto
        --
        --`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
        --
        --
        --
        --`rhs` must be of non-zero length.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` has a length of zero when `glam_assert` is enabled.

        reject_from : function(LuaVec4,LuaVec4):(LuaVec4)

        --Returns the vector projection of `self` onto `rhs`.
        --
        --
        --
        --`rhs` must be normalized.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is not normalized when `glam_assert` is enabled.

        project_onto_normalized : function(LuaVec4,LuaVec4):(LuaVec4)

        --Returns the vector rejection of `self` from `rhs`.
        --
        --
        --
        --The vector rejection is the vector perpendicular to the projection of `self` onto
        --
        --`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
        --
        --
        --
        --`rhs` must be normalized.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is not normalized when `glam_assert` is enabled.

        reject_from_normalized : function(LuaVec4,LuaVec4):(LuaVec4)

        --Returns a vector containing the nearest integer to a number for each element of `self`.
        --
        --Round half-way cases away from 0.0.

        round : function(LuaVec4):(LuaVec4)

        --Returns a vector containing the largest integer less than or equal to a number for each
        --
        --element of `self`.

        floor : function(LuaVec4):(LuaVec4)

        --Returns a vector containing the smallest integer greater than or equal to a number for
        --
        --each element of `self`.

        ceil : function(LuaVec4):(LuaVec4)

        --Returns a vector containing the integer part each element of `self`. This means numbers are
        --
        --always truncated towards zero.

        trunc : function(LuaVec4):(LuaVec4)

        --Returns a vector containing the fractional part of the vector, e.g. `self -
        --
        --self.floor()`.
        --
        --
        --
        --Note that this is fast but not precise for large numbers.

        fract : function(LuaVec4):(LuaVec4)

        --Returns a vector containing `e^self` (the exponential function) for each element of
        --
        --`self`.

        exp : function(LuaVec4):(LuaVec4)

        --Returns a vector containing each element of `self` raised to the power of `n`.

        powf : function(LuaVec4,number):(LuaVec4)

        --Returns a vector containing the reciprocal `1.0/n` of each element of `self`.

        recip : function(LuaVec4):(LuaVec4)

        --Performs a linear interpolation between `self` and `rhs` based on the value `s`.
        --
        --
        --
        --When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
        --
        --will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
        --
        --extrapolated.

        lerp : function(LuaVec4,(LuaVec4),(number)):(LuaVec4)

        --Returns true if the absolute difference of all elements between `self` and `rhs` is
        --
        --less than or equal to `max_abs_diff`.
        --
        --
        --
        --This can be used to compare if two vectors contain similar elements. It works best when
        --
        --comparing with a known value. The `max_abs_diff` that should be used used depends on
        --
        --the values being compared against.
        --
        --
        --
        --For more see
        --
        --[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

        abs_diff_eq : function(LuaVec4,(LuaVec4),(number)):(boolean)

        --Returns a vector with a length no less than `min` and no more than `max`
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `min` is greater than `max` when `glam_assert` is enabled.

        clamp_length : function(LuaVec4,(number),(number)):(LuaVec4)

        --Returns a vector with a length no more than `max`

        clamp_length_max : function(LuaVec4,number):(LuaVec4)

        --Returns a vector with a length no less than `min`

        clamp_length_min : function(LuaVec4,number):(LuaVec4)

        --Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
        --
        --error, yielding a more accurate result than an unfused multiply-add.
        --
        --
        --
        --Using `mul_add` *may* be more performant than an unfused multiply-add if the target
        --
        --architecture has a dedicated fma CPU instruction. However, this is not always true,
        --
        --and will be heavily dependant on designing algorithms with specific target hardware in
        --
        --mind.

        mul_add : function(LuaVec4,(LuaVec4),(LuaVec4)):(LuaVec4)

        --Casts all elements of `self` to `f64`.

        as_dvec4 : function(LuaVec4):(LuaDVec4)

        --Casts all elements of `self` to `i32`.

        as_ivec4 : function(LuaVec4):(LuaIVec4)

        --Casts all elements of `self` to `u32`.

        as_uvec4 : function(LuaVec4):(LuaUVec4)

        --Creates a new vector.

        new : function((number),(number),(number),(number)):(LuaVec4)

        --Creates a vector with all elements set to `v`.

        splat : function(number):(LuaVec4)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec4A),(LuaVec4),(LuaVec4)):(LuaVec4)

        __tostring : function(LuaVec4):(string)

        __unm : function(LuaVec4):(LuaVec4)

        __index : function(LuaVec4,integer):(number)

        __newindex : function(LuaVec4,(integer),(number)):()

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaVec4Class

        userdata
    
    --Collection of static methods for [`LuaVec4`].

        --Creates a new vector.

        new : function((number),(number),(number),(number)):(LuaVec4)

        --Creates a vector with all elements set to `v`.

        splat : function(number):(LuaVec4)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec4A),(LuaVec4),(LuaVec4)):(LuaVec4)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaBVec2

        userdata
    
    --

        x : boolean

        y : boolean

        --Returns a bitmask with the lowest 2 bits set from the elements of `self`.
        --
        --
        --
        --A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
        --
        --into the first lowest bit, element `y` into the second, etc.

        bitmask : function(LuaBVec2):(integer)

        --Returns true if any of the elements are true, false otherwise.

        any : function(LuaBVec2):(boolean)

        --Returns true if all the elements are true, false otherwise.

        all : function(LuaBVec2):(boolean)

        --Tests the value at `index`.
        --
        --
        --
        --Panics if `index` is greater than 1.

        test : function(LuaBVec2,integer):(boolean)

        --Sets the element at `index`.
        --
        --
        --
        --Panics if `index` is greater than 1.

        set : function(LuaBVec2,(integer),(boolean)):()

        --Creates a new vector mask.

        new : function((boolean),(boolean)):(LuaBVec2)

        --Creates a vector with all elements set to `v`.

        splat : function(boolean):(LuaBVec2)

        __tostring : function(LuaBVec2):(string)

    end
    
    
    record LuaBVec2Class

        userdata
    
    --Collection of static methods for [`LuaBVec2`].

        --Creates a new vector mask.

        new : function((boolean),(boolean)):(LuaBVec2)

        --Creates a vector with all elements set to `v`.

        splat : function(boolean):(LuaBVec2)

    end
    
    
    record LuaBVec3

        userdata
    
    --

        x : boolean

        y : boolean

        z : boolean

        --Returns a bitmask with the lowest 3 bits set from the elements of `self`.
        --
        --
        --
        --A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
        --
        --into the first lowest bit, element `y` into the second, etc.

        bitmask : function(LuaBVec3):(integer)

        --Returns true if any of the elements are true, false otherwise.

        any : function(LuaBVec3):(boolean)

        --Returns true if all the elements are true, false otherwise.

        all : function(LuaBVec3):(boolean)

        --Tests the value at `index`.
        --
        --
        --
        --Panics if `index` is greater than 2.

        test : function(LuaBVec3,integer):(boolean)

        --Sets the element at `index`.
        --
        --
        --
        --Panics if `index` is greater than 2.

        set : function(LuaBVec3,(integer),(boolean)):()

        --Creates a new vector mask.

        new : function((boolean),(boolean),(boolean)):(LuaBVec3)

        --Creates a vector with all elements set to `v`.

        splat : function(boolean):(LuaBVec3)

        __tostring : function(LuaBVec3):(string)

    end
    
    
    record LuaBVec3Class

        userdata
    
    --Collection of static methods for [`LuaBVec3`].

        --Creates a new vector mask.

        new : function((boolean),(boolean),(boolean)):(LuaBVec3)

        --Creates a vector with all elements set to `v`.

        splat : function(boolean):(LuaBVec3)

    end
    
    
    record LuaBVec4

        userdata
    
    --

        x : boolean

        y : boolean

        z : boolean

        w : boolean

        --Returns a bitmask with the lowest 4 bits set from the elements of `self`.
        --
        --
        --
        --A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
        --
        --into the first lowest bit, element `y` into the second, etc.

        bitmask : function(LuaBVec4):(integer)

        --Returns true if any of the elements are true, false otherwise.

        any : function(LuaBVec4):(boolean)

        --Returns true if all the elements are true, false otherwise.

        all : function(LuaBVec4):(boolean)

        --Tests the value at `index`.
        --
        --
        --
        --Panics if `index` is greater than 3.

        test : function(LuaBVec4,integer):(boolean)

        --Sets the element at `index`.
        --
        --
        --
        --Panics if `index` is greater than 3.

        set : function(LuaBVec4,(integer),(boolean)):()

        --Creates a new vector mask.

        new : function((boolean),(boolean),(boolean),(boolean)):(LuaBVec4)

        --Creates a vector with all elements set to `v`.

        splat : function(boolean):(LuaBVec4)

        __tostring : function(LuaBVec4):(string)

    end
    
    
    record LuaBVec4Class

        userdata
    
    --Collection of static methods for [`LuaBVec4`].

        --Creates a new vector mask.

        new : function((boolean),(boolean),(boolean),(boolean)):(LuaBVec4)

        --Creates a vector with all elements set to `v`.

        splat : function(boolean):(LuaBVec4)

    end
    
    
    record LuaBVec3A

        userdata
    
    --

        --Returns a bitmask with the lowest 3 bits set from the elements of `self`.
        --
        --
        --
        --A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
        --
        --into the first lowest bit, element `y` into the second, etc.

        bitmask : function(LuaBVec3A):(integer)

        --Returns true if any of the elements are true, false otherwise.

        any : function(LuaBVec3A):(boolean)

        --Returns true if all the elements are true, false otherwise.

        all : function(LuaBVec3A):(boolean)

        --Tests the value at `index`.
        --
        --
        --
        --Panics if `index` is greater than 2.

        test : function(LuaBVec3A,integer):(boolean)

        --Sets the element at `index`.
        --
        --
        --
        --Panics if `index` is greater than 2.

        set : function(LuaBVec3A,(integer),(boolean)):()

        --Creates a new vector mask.

        new : function((boolean),(boolean),(boolean)):(LuaBVec3A)

        --Creates a vector with all elements set to `v`.

        splat : function(boolean):(LuaBVec3A)

        __tostring : function(LuaBVec3A):(string)

    end
    
    
    record LuaBVec3AClass

        userdata
    
    --Collection of static methods for [`LuaBVec3A`].

        --Creates a new vector mask.

        new : function((boolean),(boolean),(boolean)):(LuaBVec3A)

        --Creates a vector with all elements set to `v`.

        splat : function(boolean):(LuaBVec3A)

    end
    
    
    record LuaBVec4A

        userdata
    
    --

        --Returns a bitmask with the lowest 4 bits set from the elements of `self`.
        --
        --
        --
        --A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
        --
        --into the first lowest bit, element `y` into the second, etc.

        bitmask : function(LuaBVec4A):(integer)

        --Returns true if any of the elements are true, false otherwise.

        any : function(LuaBVec4A):(boolean)

        --Returns true if all the elements are true, false otherwise.

        all : function(LuaBVec4A):(boolean)

        --Tests the value at `index`.
        --
        --
        --
        --Panics if `index` is greater than 3.

        test : function(LuaBVec4A,integer):(boolean)

        --Sets the element at `index`.
        --
        --
        --
        --Panics if `index` is greater than 3.

        set : function(LuaBVec4A,(integer),(boolean)):()

        --Creates a new vector mask.

        new : function((boolean),(boolean),(boolean),(boolean)):(LuaBVec4A)

        --Creates a vector with all elements set to `v`.

        splat : function(boolean):(LuaBVec4A)

        __tostring : function(LuaBVec4A):(string)

    end
    
    
    record LuaBVec4AClass

        userdata
    
    --Collection of static methods for [`LuaBVec4A`].

        --Creates a new vector mask.

        new : function((boolean),(boolean),(boolean),(boolean)):(LuaBVec4A)

        --Creates a vector with all elements set to `v`.

        splat : function(boolean):(LuaBVec4A)

    end
    
    
    record LuaDVec2

        userdata
    
    --

        x : number

        y : number

        --Creates a 3D vector from `self` and the given `z` value.

        extend : function(LuaDVec2,number):(LuaDVec3)

        --Computes the dot product of `self` and `rhs`.

        dot : function(LuaDVec2,LuaDVec2):(number)

        --Returns a vector where every component is the dot product of `self` and `rhs`.

        dot_into_vec : function(LuaDVec2,LuaDVec2):(LuaDVec2)

        --Returns a vector containing the minimum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

        min : function(LuaDVec2,LuaDVec2):(LuaDVec2)

        --Returns a vector containing the maximum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

        max : function(LuaDVec2,LuaDVec2):(LuaDVec2)

        --Component-wise clamping of values, similar to [`f64::clamp`].
        --
        --
        --
        --Each element in `min` must be less-or-equal to the corresponding element in `max`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `min` is greater than `max` when `glam_assert` is enabled.

        clamp : function(LuaDVec2,(LuaDVec2),(LuaDVec2)):(LuaDVec2)

        --Returns the horizontal minimum of `self`.
        --
        --
        --
        --In other words this computes `min(x, y, ..)`.

        min_element : function(LuaDVec2):(number)

        --Returns the horizontal maximum of `self`.
        --
        --
        --
        --In other words this computes `max(x, y, ..)`.

        max_element : function(LuaDVec2):(number)

        --Returns a vector mask containing the result of a `==` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
        --
        --elements.

        cmpeq : function(LuaDVec2,LuaDVec2):(LuaBVec2)

        --Returns a vector mask containing the result of a `!=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
        --
        --elements.

        cmpne : function(LuaDVec2,LuaDVec2):(LuaBVec2)

        --Returns a vector mask containing the result of a `>=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
        --
        --elements.

        cmpge : function(LuaDVec2,LuaDVec2):(LuaBVec2)

        --Returns a vector mask containing the result of a `>` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
        --
        --elements.

        cmpgt : function(LuaDVec2,LuaDVec2):(LuaBVec2)

        --Returns a vector mask containing the result of a `<=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
        --
        --elements.

        cmple : function(LuaDVec2,LuaDVec2):(LuaBVec2)

        --Returns a vector mask containing the result of a `<` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
        --
        --elements.

        cmplt : function(LuaDVec2,LuaDVec2):(LuaBVec2)

        --Returns a vector containing the absolute value of each element of `self`.

        abs : function(LuaDVec2):(LuaDVec2)

        --Returns a vector with elements representing the sign of `self`.
        --
        --
        --
        --- `1.0` if the number is positive, `+0.0` or `INFINITY`
        --
        --- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
        --
        --- `NAN` if the number is `NAN`

        signum : function(LuaDVec2):(LuaDVec2)

        --Returns a vector with signs of `rhs` and the magnitudes of `self`.

        copysign : function(LuaDVec2,LuaDVec2):(LuaDVec2)

        --Returns a bitmask with the lowest 2 bits set to the sign bits from the elements of `self`.
        --
        --
        --
        --A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
        --
        --into the first lowest bit, element `y` into the second, etc.

        is_negative_bitmask : function(LuaDVec2):(integer)

        --Returns `true` if, and only if, all elements are finite.  If any element is either
        --
        --`NaN`, positive or negative infinity, this will return `false`.

        is_finite : function(LuaDVec2):(boolean)

        --Returns `true` if any elements are `NaN`.

        is_nan : function(LuaDVec2):(boolean)

        --Performs `is_nan` on each element of self, returning a vector mask of the results.
        --
        --
        --
        --In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.

        is_nan_mask : function(LuaDVec2):(LuaBVec2)

        --Computes the length of `self`.

        length : function(LuaDVec2):(number)

        --Computes the squared length of `self`.
        --
        --
        --
        --This is faster than `length()` as it avoids a square root operation.

        length_squared : function(LuaDVec2):(number)

        --Computes `1.0 / length()`.
        --
        --
        --
        --For valid results, `self` must _not_ be of length zero.

        length_recip : function(LuaDVec2):(number)

        --Computes the Euclidean distance between two points in space.

        distance : function(LuaDVec2,LuaDVec2):(number)

        --Compute the squared euclidean distance between two points in space.

        distance_squared : function(LuaDVec2,LuaDVec2):(number)

        --Returns `self` normalized to length 1.0.
        --
        --
        --
        --For valid results, `self` must _not_ be of length zero, nor very close to zero.
        --
        --
        --
        --See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
        --
        --
        --
        --Panics
        --
        --
        --
        --Will panic if `self` is zero length when `glam_assert` is enabled.

        normalize : function(LuaDVec2):(LuaDVec2)

        --Returns `self` normalized to length 1.0 if possible, else returns zero.
        --
        --
        --
        --In particular, if the input is zero (or very close to zero), or non-finite,
        --
        --the result of this operation will be zero.
        --
        --
        --
        --See also [`Self::try_normalize()`].

        normalize_or_zero : function(LuaDVec2):(LuaDVec2)

        --Returns whether `self` is length `1.0` or not.
        --
        --
        --
        --Uses a precision threshold of `1e-6`.

        is_normalized : function(LuaDVec2):(boolean)

        --Returns the vector projection of `self` onto `rhs`.
        --
        --
        --
        --`rhs` must be of non-zero length.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is zero length when `glam_assert` is enabled.

        project_onto : function(LuaDVec2,LuaDVec2):(LuaDVec2)

        --Returns the vector rejection of `self` from `rhs`.
        --
        --
        --
        --The vector rejection is the vector perpendicular to the projection of `self` onto
        --
        --`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
        --
        --
        --
        --`rhs` must be of non-zero length.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` has a length of zero when `glam_assert` is enabled.

        reject_from : function(LuaDVec2,LuaDVec2):(LuaDVec2)

        --Returns the vector projection of `self` onto `rhs`.
        --
        --
        --
        --`rhs` must be normalized.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is not normalized when `glam_assert` is enabled.

        project_onto_normalized : function(LuaDVec2,LuaDVec2):(LuaDVec2)

        --Returns the vector rejection of `self` from `rhs`.
        --
        --
        --
        --The vector rejection is the vector perpendicular to the projection of `self` onto
        --
        --`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
        --
        --
        --
        --`rhs` must be normalized.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is not normalized when `glam_assert` is enabled.

        reject_from_normalized : function(LuaDVec2,LuaDVec2):(LuaDVec2)

        --Returns a vector containing the nearest integer to a number for each element of `self`.
        --
        --Round half-way cases away from 0.0.

        round : function(LuaDVec2):(LuaDVec2)

        --Returns a vector containing the largest integer less than or equal to a number for each
        --
        --element of `self`.

        floor : function(LuaDVec2):(LuaDVec2)

        --Returns a vector containing the smallest integer greater than or equal to a number for
        --
        --each element of `self`.

        ceil : function(LuaDVec2):(LuaDVec2)

        --Returns a vector containing the integer part each element of `self`. This means numbers are
        --
        --always truncated towards zero.

        trunc : function(LuaDVec2):(LuaDVec2)

        --Returns a vector containing the fractional part of the vector, e.g. `self -
        --
        --self.floor()`.
        --
        --
        --
        --Note that this is fast but not precise for large numbers.

        fract : function(LuaDVec2):(LuaDVec2)

        --Returns a vector containing `e^self` (the exponential function) for each element of
        --
        --`self`.

        exp : function(LuaDVec2):(LuaDVec2)

        --Returns a vector containing each element of `self` raised to the power of `n`.

        powf : function(LuaDVec2,number):(LuaDVec2)

        --Returns a vector containing the reciprocal `1.0/n` of each element of `self`.

        recip : function(LuaDVec2):(LuaDVec2)

        --Performs a linear interpolation between `self` and `rhs` based on the value `s`.
        --
        --
        --
        --When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
        --
        --will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
        --
        --extrapolated.

        lerp : function(LuaDVec2,(LuaDVec2),(number)):(LuaDVec2)

        --Returns true if the absolute difference of all elements between `self` and `rhs` is
        --
        --less than or equal to `max_abs_diff`.
        --
        --
        --
        --This can be used to compare if two vectors contain similar elements. It works best when
        --
        --comparing with a known value. The `max_abs_diff` that should be used used depends on
        --
        --the values being compared against.
        --
        --
        --
        --For more see
        --
        --[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

        abs_diff_eq : function(LuaDVec2,(LuaDVec2),(number)):(boolean)

        --Returns a vector with a length no less than `min` and no more than `max`
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `min` is greater than `max` when `glam_assert` is enabled.

        clamp_length : function(LuaDVec2,(number),(number)):(LuaDVec2)

        --Returns a vector with a length no more than `max`

        clamp_length_max : function(LuaDVec2,number):(LuaDVec2)

        --Returns a vector with a length no less than `min`

        clamp_length_min : function(LuaDVec2,number):(LuaDVec2)

        --Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
        --
        --error, yielding a more accurate result than an unfused multiply-add.
        --
        --
        --
        --Using `mul_add` *may* be more performant than an unfused multiply-add if the target
        --
        --architecture has a dedicated fma CPU instruction. However, this is not always true,
        --
        --and will be heavily dependant on designing algorithms with specific target hardware in
        --
        --mind.

        mul_add : function(LuaDVec2,(LuaDVec2),(LuaDVec2)):(LuaDVec2)

        --Returns the angle (in radians) between `self` and `rhs` in the range `[-π, +π]`.
        --
        --
        --
        --The inputs do not need to be unit vectors however they must be non-zero.

        angle_between : function(LuaDVec2,LuaDVec2):(number)

        --Returns a vector that is equal to `self` rotated by 90 degrees.

        perp : function(LuaDVec2):(LuaDVec2)

        --The perpendicular dot product of `self` and `rhs`.
        --
        --Also known as the wedge product, 2D cross product, and determinant.

        perp_dot : function(LuaDVec2,LuaDVec2):(number)

        --Returns `rhs` rotated by the angle of `self`. If `self` is normalized,
        --
        --then this just rotation. This is what you usually want. Otherwise,
        --
        --it will be like a rotation with a multiplication by `self`'s length.

        rotate : function(LuaDVec2,LuaDVec2):(LuaDVec2)

        --Casts all elements of `self` to `f32`.

        as_vec2 : function(LuaDVec2):(LuaVec2)

        --Casts all elements of `self` to `i32`.

        as_ivec2 : function(LuaDVec2):(LuaIVec2)

        --Casts all elements of `self` to `u32`.

        as_uvec2 : function(LuaDVec2):(LuaUVec2)

        --Creates a new vector.

        new : function((number),(number)):(LuaDVec2)

        --Creates a vector with all elements set to `v`.

        splat : function(number):(LuaDVec2)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec2),(LuaDVec2),(LuaDVec2)):(LuaDVec2)

        --Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in
        --
        --conjunction with the [`rotate()`][Self::rotate()] method, e.g.
        --
        --`DVec2::from_angle(PI).rotate(DVec2::Y)` will create the vector `[-1, 0]`
        --
        --and rotate [`DVec2::Y`] around it returning `-DVec2::Y`.

        from_angle : function(number):(LuaDVec2)

        __tostring : function(LuaDVec2):(string)

        __unm : function(LuaDVec2):(LuaDVec2)

        __index : function(LuaDVec2,integer):(number)

        __newindex : function(LuaDVec2,(integer),(number)):()

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaDVec2Class

        userdata
    
    --Collection of static methods for [`LuaDVec2`].

        --Creates a new vector.

        new : function((number),(number)):(LuaDVec2)

        --Creates a vector with all elements set to `v`.

        splat : function(number):(LuaDVec2)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec2),(LuaDVec2),(LuaDVec2)):(LuaDVec2)

        --Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in
        --
        --conjunction with the [`rotate()`][Self::rotate()] method, e.g.
        --
        --`DVec2::from_angle(PI).rotate(DVec2::Y)` will create the vector `[-1, 0]`
        --
        --and rotate [`DVec2::Y`] around it returning `-DVec2::Y`.

        from_angle : function(number):(LuaDVec2)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaDVec3

        userdata
    
    --

        x : number

        y : number

        z : number

        --Creates a 4D vector from `self` and the given `w` value.

        extend : function(LuaDVec3,number):(LuaDVec4)

        --Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
        --
        --
        --
        --Truncation may also be performed by using [`self.xy()`][crate::swizzles::Vec3Swizzles::xy()].

        truncate : function(LuaDVec3):(LuaDVec2)

        --Computes the dot product of `self` and `rhs`.

        dot : function(LuaDVec3,LuaDVec3):(number)

        --Returns a vector where every component is the dot product of `self` and `rhs`.

        dot_into_vec : function(LuaDVec3,LuaDVec3):(LuaDVec3)

        --Computes the cross product of `self` and `rhs`.

        cross : function(LuaDVec3,LuaDVec3):(LuaDVec3)

        --Returns a vector containing the minimum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

        min : function(LuaDVec3,LuaDVec3):(LuaDVec3)

        --Returns a vector containing the maximum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

        max : function(LuaDVec3,LuaDVec3):(LuaDVec3)

        --Component-wise clamping of values, similar to [`f64::clamp`].
        --
        --
        --
        --Each element in `min` must be less-or-equal to the corresponding element in `max`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `min` is greater than `max` when `glam_assert` is enabled.

        clamp : function(LuaDVec3,(LuaDVec3),(LuaDVec3)):(LuaDVec3)

        --Returns the horizontal minimum of `self`.
        --
        --
        --
        --In other words this computes `min(x, y, ..)`.

        min_element : function(LuaDVec3):(number)

        --Returns the horizontal maximum of `self`.
        --
        --
        --
        --In other words this computes `max(x, y, ..)`.

        max_element : function(LuaDVec3):(number)

        --Returns a vector mask containing the result of a `==` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
        --
        --elements.

        cmpeq : function(LuaDVec3,LuaDVec3):(LuaBVec3)

        --Returns a vector mask containing the result of a `!=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
        --
        --elements.

        cmpne : function(LuaDVec3,LuaDVec3):(LuaBVec3)

        --Returns a vector mask containing the result of a `>=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
        --
        --elements.

        cmpge : function(LuaDVec3,LuaDVec3):(LuaBVec3)

        --Returns a vector mask containing the result of a `>` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
        --
        --elements.

        cmpgt : function(LuaDVec3,LuaDVec3):(LuaBVec3)

        --Returns a vector mask containing the result of a `<=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
        --
        --elements.

        cmple : function(LuaDVec3,LuaDVec3):(LuaBVec3)

        --Returns a vector mask containing the result of a `<` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
        --
        --elements.

        cmplt : function(LuaDVec3,LuaDVec3):(LuaBVec3)

        --Returns a vector containing the absolute value of each element of `self`.

        abs : function(LuaDVec3):(LuaDVec3)

        --Returns a vector with elements representing the sign of `self`.
        --
        --
        --
        --- `1.0` if the number is positive, `+0.0` or `INFINITY`
        --
        --- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
        --
        --- `NAN` if the number is `NAN`

        signum : function(LuaDVec3):(LuaDVec3)

        --Returns a vector with signs of `rhs` and the magnitudes of `self`.

        copysign : function(LuaDVec3,LuaDVec3):(LuaDVec3)

        --Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of `self`.
        --
        --
        --
        --A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
        --
        --into the first lowest bit, element `y` into the second, etc.

        is_negative_bitmask : function(LuaDVec3):(integer)

        --Returns `true` if, and only if, all elements are finite.  If any element is either
        --
        --`NaN`, positive or negative infinity, this will return `false`.

        is_finite : function(LuaDVec3):(boolean)

        --Returns `true` if any elements are `NaN`.

        is_nan : function(LuaDVec3):(boolean)

        --Performs `is_nan` on each element of self, returning a vector mask of the results.
        --
        --
        --
        --In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.

        is_nan_mask : function(LuaDVec3):(LuaBVec3)

        --Computes the length of `self`.

        length : function(LuaDVec3):(number)

        --Computes the squared length of `self`.
        --
        --
        --
        --This is faster than `length()` as it avoids a square root operation.

        length_squared : function(LuaDVec3):(number)

        --Computes `1.0 / length()`.
        --
        --
        --
        --For valid results, `self` must _not_ be of length zero.

        length_recip : function(LuaDVec3):(number)

        --Computes the Euclidean distance between two points in space.

        distance : function(LuaDVec3,LuaDVec3):(number)

        --Compute the squared euclidean distance between two points in space.

        distance_squared : function(LuaDVec3,LuaDVec3):(number)

        --Returns `self` normalized to length 1.0.
        --
        --
        --
        --For valid results, `self` must _not_ be of length zero, nor very close to zero.
        --
        --
        --
        --See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
        --
        --
        --
        --Panics
        --
        --
        --
        --Will panic if `self` is zero length when `glam_assert` is enabled.

        normalize : function(LuaDVec3):(LuaDVec3)

        --Returns `self` normalized to length 1.0 if possible, else returns zero.
        --
        --
        --
        --In particular, if the input is zero (or very close to zero), or non-finite,
        --
        --the result of this operation will be zero.
        --
        --
        --
        --See also [`Self::try_normalize()`].

        normalize_or_zero : function(LuaDVec3):(LuaDVec3)

        --Returns whether `self` is length `1.0` or not.
        --
        --
        --
        --Uses a precision threshold of `1e-6`.

        is_normalized : function(LuaDVec3):(boolean)

        --Returns the vector projection of `self` onto `rhs`.
        --
        --
        --
        --`rhs` must be of non-zero length.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is zero length when `glam_assert` is enabled.

        project_onto : function(LuaDVec3,LuaDVec3):(LuaDVec3)

        --Returns the vector rejection of `self` from `rhs`.
        --
        --
        --
        --The vector rejection is the vector perpendicular to the projection of `self` onto
        --
        --`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
        --
        --
        --
        --`rhs` must be of non-zero length.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` has a length of zero when `glam_assert` is enabled.

        reject_from : function(LuaDVec3,LuaDVec3):(LuaDVec3)

        --Returns the vector projection of `self` onto `rhs`.
        --
        --
        --
        --`rhs` must be normalized.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is not normalized when `glam_assert` is enabled.

        project_onto_normalized : function(LuaDVec3,LuaDVec3):(LuaDVec3)

        --Returns the vector rejection of `self` from `rhs`.
        --
        --
        --
        --The vector rejection is the vector perpendicular to the projection of `self` onto
        --
        --`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
        --
        --
        --
        --`rhs` must be normalized.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is not normalized when `glam_assert` is enabled.

        reject_from_normalized : function(LuaDVec3,LuaDVec3):(LuaDVec3)

        --Returns a vector containing the nearest integer to a number for each element of `self`.
        --
        --Round half-way cases away from 0.0.

        round : function(LuaDVec3):(LuaDVec3)

        --Returns a vector containing the largest integer less than or equal to a number for each
        --
        --element of `self`.

        floor : function(LuaDVec3):(LuaDVec3)

        --Returns a vector containing the smallest integer greater than or equal to a number for
        --
        --each element of `self`.

        ceil : function(LuaDVec3):(LuaDVec3)

        --Returns a vector containing the integer part each element of `self`. This means numbers are
        --
        --always truncated towards zero.

        trunc : function(LuaDVec3):(LuaDVec3)

        --Returns a vector containing the fractional part of the vector, e.g. `self -
        --
        --self.floor()`.
        --
        --
        --
        --Note that this is fast but not precise for large numbers.

        fract : function(LuaDVec3):(LuaDVec3)

        --Returns a vector containing `e^self` (the exponential function) for each element of
        --
        --`self`.

        exp : function(LuaDVec3):(LuaDVec3)

        --Returns a vector containing each element of `self` raised to the power of `n`.

        powf : function(LuaDVec3,number):(LuaDVec3)

        --Returns a vector containing the reciprocal `1.0/n` of each element of `self`.

        recip : function(LuaDVec3):(LuaDVec3)

        --Performs a linear interpolation between `self` and `rhs` based on the value `s`.
        --
        --
        --
        --When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
        --
        --will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
        --
        --extrapolated.

        lerp : function(LuaDVec3,(LuaDVec3),(number)):(LuaDVec3)

        --Returns true if the absolute difference of all elements between `self` and `rhs` is
        --
        --less than or equal to `max_abs_diff`.
        --
        --
        --
        --This can be used to compare if two vectors contain similar elements. It works best when
        --
        --comparing with a known value. The `max_abs_diff` that should be used used depends on
        --
        --the values being compared against.
        --
        --
        --
        --For more see
        --
        --[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

        abs_diff_eq : function(LuaDVec3,(LuaDVec3),(number)):(boolean)

        --Returns a vector with a length no less than `min` and no more than `max`
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `min` is greater than `max` when `glam_assert` is enabled.

        clamp_length : function(LuaDVec3,(number),(number)):(LuaDVec3)

        --Returns a vector with a length no more than `max`

        clamp_length_max : function(LuaDVec3,number):(LuaDVec3)

        --Returns a vector with a length no less than `min`

        clamp_length_min : function(LuaDVec3,number):(LuaDVec3)

        --Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
        --
        --error, yielding a more accurate result than an unfused multiply-add.
        --
        --
        --
        --Using `mul_add` *may* be more performant than an unfused multiply-add if the target
        --
        --architecture has a dedicated fma CPU instruction. However, this is not always true,
        --
        --and will be heavily dependant on designing algorithms with specific target hardware in
        --
        --mind.

        mul_add : function(LuaDVec3,(LuaDVec3),(LuaDVec3)):(LuaDVec3)

        --Returns the angle (in radians) between two vectors.
        --
        --
        --
        --The inputs do not need to be unit vectors however they must be non-zero.

        angle_between : function(LuaDVec3,LuaDVec3):(number)

        --Returns some vector that is orthogonal to the given one.
        --
        --
        --
        --The input vector must be finite and non-zero.
        --
        --
        --
        --The output vector is not necessarily unit length. For that use
        --
        --[`Self::any_orthonormal_vector()`] instead.

        any_orthogonal_vector : function(LuaDVec3):(LuaDVec3)

        --Returns any unit vector that is orthogonal to the given one.
        --
        --
        --
        --The input vector must be unit length.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `self` is not normalized when `glam_assert` is enabled.

        any_orthonormal_vector : function(LuaDVec3):(LuaDVec3)

        --Casts all elements of `self` to `f32`.

        as_vec3 : function(LuaDVec3):(LuaVec3)

        --Casts all elements of `self` to `f32`.

        as_vec3a : function(LuaDVec3):(LuaVec3A)

        --Casts all elements of `self` to `i32`.

        as_ivec3 : function(LuaDVec3):(LuaIVec3)

        --Casts all elements of `self` to `u32`.

        as_uvec3 : function(LuaDVec3):(LuaUVec3)

        --Creates a new vector.

        new : function((number),(number),(number)):(LuaDVec3)

        --Creates a vector with all elements set to `v`.

        splat : function(number):(LuaDVec3)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec3),(LuaDVec3),(LuaDVec3)):(LuaDVec3)

        __tostring : function(LuaDVec3):(string)

        __unm : function(LuaDVec3):(LuaDVec3)

        __index : function(LuaDVec3,integer):(number)

        __newindex : function(LuaDVec3,(integer),(number)):()

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaDVec3Class

        userdata
    
    --Collection of static methods for [`LuaDVec3`].

        --Creates a new vector.

        new : function((number),(number),(number)):(LuaDVec3)

        --Creates a vector with all elements set to `v`.

        splat : function(number):(LuaDVec3)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec3),(LuaDVec3),(LuaDVec3)):(LuaDVec3)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaDVec4

        userdata
    
    --

        x : number

        y : number

        z : number

        w : number

        --Creates a 2D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
        --
        --
        --
        --Truncation to [`DVec3`] may also be performed by using [`self.xyz()`][crate::swizzles::Vec4Swizzles::xyz()].

        truncate : function(LuaDVec4):(LuaDVec3)

        --Computes the dot product of `self` and `rhs`.

        dot : function(LuaDVec4,LuaDVec4):(number)

        --Returns a vector where every component is the dot product of `self` and `rhs`.

        dot_into_vec : function(LuaDVec4,LuaDVec4):(LuaDVec4)

        --Returns a vector containing the minimum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

        min : function(LuaDVec4,LuaDVec4):(LuaDVec4)

        --Returns a vector containing the maximum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

        max : function(LuaDVec4,LuaDVec4):(LuaDVec4)

        --Component-wise clamping of values, similar to [`f64::clamp`].
        --
        --
        --
        --Each element in `min` must be less-or-equal to the corresponding element in `max`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `min` is greater than `max` when `glam_assert` is enabled.

        clamp : function(LuaDVec4,(LuaDVec4),(LuaDVec4)):(LuaDVec4)

        --Returns the horizontal minimum of `self`.
        --
        --
        --
        --In other words this computes `min(x, y, ..)`.

        min_element : function(LuaDVec4):(number)

        --Returns the horizontal maximum of `self`.
        --
        --
        --
        --In other words this computes `max(x, y, ..)`.

        max_element : function(LuaDVec4):(number)

        --Returns a vector mask containing the result of a `==` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
        --
        --elements.

        cmpeq : function(LuaDVec4,LuaDVec4):(LuaBVec4)

        --Returns a vector mask containing the result of a `!=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
        --
        --elements.

        cmpne : function(LuaDVec4,LuaDVec4):(LuaBVec4)

        --Returns a vector mask containing the result of a `>=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
        --
        --elements.

        cmpge : function(LuaDVec4,LuaDVec4):(LuaBVec4)

        --Returns a vector mask containing the result of a `>` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
        --
        --elements.

        cmpgt : function(LuaDVec4,LuaDVec4):(LuaBVec4)

        --Returns a vector mask containing the result of a `<=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
        --
        --elements.

        cmple : function(LuaDVec4,LuaDVec4):(LuaBVec4)

        --Returns a vector mask containing the result of a `<` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
        --
        --elements.

        cmplt : function(LuaDVec4,LuaDVec4):(LuaBVec4)

        --Returns a vector containing the absolute value of each element of `self`.

        abs : function(LuaDVec4):(LuaDVec4)

        --Returns a vector with elements representing the sign of `self`.
        --
        --
        --
        --- `1.0` if the number is positive, `+0.0` or `INFINITY`
        --
        --- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
        --
        --- `NAN` if the number is `NAN`

        signum : function(LuaDVec4):(LuaDVec4)

        --Returns a vector with signs of `rhs` and the magnitudes of `self`.

        copysign : function(LuaDVec4,LuaDVec4):(LuaDVec4)

        --Returns a bitmask with the lowest 4 bits set to the sign bits from the elements of `self`.
        --
        --
        --
        --A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
        --
        --into the first lowest bit, element `y` into the second, etc.

        is_negative_bitmask : function(LuaDVec4):(integer)

        --Returns `true` if, and only if, all elements are finite.  If any element is either
        --
        --`NaN`, positive or negative infinity, this will return `false`.

        is_finite : function(LuaDVec4):(boolean)

        --Returns `true` if any elements are `NaN`.

        is_nan : function(LuaDVec4):(boolean)

        --Performs `is_nan` on each element of self, returning a vector mask of the results.
        --
        --
        --
        --In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.

        is_nan_mask : function(LuaDVec4):(LuaBVec4)

        --Computes the length of `self`.

        length : function(LuaDVec4):(number)

        --Computes the squared length of `self`.
        --
        --
        --
        --This is faster than `length()` as it avoids a square root operation.

        length_squared : function(LuaDVec4):(number)

        --Computes `1.0 / length()`.
        --
        --
        --
        --For valid results, `self` must _not_ be of length zero.

        length_recip : function(LuaDVec4):(number)

        --Computes the Euclidean distance between two points in space.

        distance : function(LuaDVec4,LuaDVec4):(number)

        --Compute the squared euclidean distance between two points in space.

        distance_squared : function(LuaDVec4,LuaDVec4):(number)

        --Returns `self` normalized to length 1.0.
        --
        --
        --
        --For valid results, `self` must _not_ be of length zero, nor very close to zero.
        --
        --
        --
        --See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
        --
        --
        --
        --Panics
        --
        --
        --
        --Will panic if `self` is zero length when `glam_assert` is enabled.

        normalize : function(LuaDVec4):(LuaDVec4)

        --Returns `self` normalized to length 1.0 if possible, else returns zero.
        --
        --
        --
        --In particular, if the input is zero (or very close to zero), or non-finite,
        --
        --the result of this operation will be zero.
        --
        --
        --
        --See also [`Self::try_normalize()`].

        normalize_or_zero : function(LuaDVec4):(LuaDVec4)

        --Returns whether `self` is length `1.0` or not.
        --
        --
        --
        --Uses a precision threshold of `1e-6`.

        is_normalized : function(LuaDVec4):(boolean)

        --Returns the vector projection of `self` onto `rhs`.
        --
        --
        --
        --`rhs` must be of non-zero length.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is zero length when `glam_assert` is enabled.

        project_onto : function(LuaDVec4,LuaDVec4):(LuaDVec4)

        --Returns the vector rejection of `self` from `rhs`.
        --
        --
        --
        --The vector rejection is the vector perpendicular to the projection of `self` onto
        --
        --`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
        --
        --
        --
        --`rhs` must be of non-zero length.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` has a length of zero when `glam_assert` is enabled.

        reject_from : function(LuaDVec4,LuaDVec4):(LuaDVec4)

        --Returns the vector projection of `self` onto `rhs`.
        --
        --
        --
        --`rhs` must be normalized.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is not normalized when `glam_assert` is enabled.

        project_onto_normalized : function(LuaDVec4,LuaDVec4):(LuaDVec4)

        --Returns the vector rejection of `self` from `rhs`.
        --
        --
        --
        --The vector rejection is the vector perpendicular to the projection of `self` onto
        --
        --`rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
        --
        --
        --
        --`rhs` must be normalized.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rhs` is not normalized when `glam_assert` is enabled.

        reject_from_normalized : function(LuaDVec4,LuaDVec4):(LuaDVec4)

        --Returns a vector containing the nearest integer to a number for each element of `self`.
        --
        --Round half-way cases away from 0.0.

        round : function(LuaDVec4):(LuaDVec4)

        --Returns a vector containing the largest integer less than or equal to a number for each
        --
        --element of `self`.

        floor : function(LuaDVec4):(LuaDVec4)

        --Returns a vector containing the smallest integer greater than or equal to a number for
        --
        --each element of `self`.

        ceil : function(LuaDVec4):(LuaDVec4)

        --Returns a vector containing the integer part each element of `self`. This means numbers are
        --
        --always truncated towards zero.

        trunc : function(LuaDVec4):(LuaDVec4)

        --Returns a vector containing the fractional part of the vector, e.g. `self -
        --
        --self.floor()`.
        --
        --
        --
        --Note that this is fast but not precise for large numbers.

        fract : function(LuaDVec4):(LuaDVec4)

        --Returns a vector containing `e^self` (the exponential function) for each element of
        --
        --`self`.

        exp : function(LuaDVec4):(LuaDVec4)

        --Returns a vector containing each element of `self` raised to the power of `n`.

        powf : function(LuaDVec4,number):(LuaDVec4)

        --Returns a vector containing the reciprocal `1.0/n` of each element of `self`.

        recip : function(LuaDVec4):(LuaDVec4)

        --Performs a linear interpolation between `self` and `rhs` based on the value `s`.
        --
        --
        --
        --When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
        --
        --will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
        --
        --extrapolated.

        lerp : function(LuaDVec4,(LuaDVec4),(number)):(LuaDVec4)

        --Returns true if the absolute difference of all elements between `self` and `rhs` is
        --
        --less than or equal to `max_abs_diff`.
        --
        --
        --
        --This can be used to compare if two vectors contain similar elements. It works best when
        --
        --comparing with a known value. The `max_abs_diff` that should be used used depends on
        --
        --the values being compared against.
        --
        --
        --
        --For more see
        --
        --[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

        abs_diff_eq : function(LuaDVec4,(LuaDVec4),(number)):(boolean)

        --Returns a vector with a length no less than `min` and no more than `max`
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `min` is greater than `max` when `glam_assert` is enabled.

        clamp_length : function(LuaDVec4,(number),(number)):(LuaDVec4)

        --Returns a vector with a length no more than `max`

        clamp_length_max : function(LuaDVec4,number):(LuaDVec4)

        --Returns a vector with a length no less than `min`

        clamp_length_min : function(LuaDVec4,number):(LuaDVec4)

        --Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
        --
        --error, yielding a more accurate result than an unfused multiply-add.
        --
        --
        --
        --Using `mul_add` *may* be more performant than an unfused multiply-add if the target
        --
        --architecture has a dedicated fma CPU instruction. However, this is not always true,
        --
        --and will be heavily dependant on designing algorithms with specific target hardware in
        --
        --mind.

        mul_add : function(LuaDVec4,(LuaDVec4),(LuaDVec4)):(LuaDVec4)

        --Casts all elements of `self` to `f32`.

        as_vec4 : function(LuaDVec4):(LuaVec4)

        --Casts all elements of `self` to `i32`.

        as_ivec4 : function(LuaDVec4):(LuaIVec4)

        --Casts all elements of `self` to `u32`.

        as_uvec4 : function(LuaDVec4):(LuaUVec4)

        --Creates a new vector.

        new : function((number),(number),(number),(number)):(LuaDVec4)

        --Creates a vector with all elements set to `v`.

        splat : function(number):(LuaDVec4)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec4),(LuaDVec4),(LuaDVec4)):(LuaDVec4)

        __tostring : function(LuaDVec4):(string)

        __unm : function(LuaDVec4):(LuaDVec4)

        __index : function(LuaDVec4,integer):(number)

        __newindex : function(LuaDVec4,(integer),(number)):()

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaDVec4Class

        userdata
    
    --Collection of static methods for [`LuaDVec4`].

        --Creates a new vector.

        new : function((number),(number),(number),(number)):(LuaDVec4)

        --Creates a vector with all elements set to `v`.

        splat : function(number):(LuaDVec4)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec4),(LuaDVec4),(LuaDVec4)):(LuaDVec4)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaIVec2

        userdata
    
    --

        x : integer

        y : integer

        --Creates a 3D vector from `self` and the given `z` value.

        extend : function(LuaIVec2,integer):(LuaIVec3)

        --Computes the dot product of `self` and `rhs`.

        dot : function(LuaIVec2,LuaIVec2):(integer)

        --Returns a vector where every component is the dot product of `self` and `rhs`.

        dot_into_vec : function(LuaIVec2,LuaIVec2):(LuaIVec2)

        --Returns a vector containing the minimum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

        min : function(LuaIVec2,LuaIVec2):(LuaIVec2)

        --Returns a vector containing the maximum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

        max : function(LuaIVec2,LuaIVec2):(LuaIVec2)

        --Component-wise clamping of values, similar to [`i32::clamp`].
        --
        --
        --
        --Each element in `min` must be less-or-equal to the corresponding element in `max`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `min` is greater than `max` when `glam_assert` is enabled.

        clamp : function(LuaIVec2,(LuaIVec2),(LuaIVec2)):(LuaIVec2)

        --Returns the horizontal minimum of `self`.
        --
        --
        --
        --In other words this computes `min(x, y, ..)`.

        min_element : function(LuaIVec2):(integer)

        --Returns the horizontal maximum of `self`.
        --
        --
        --
        --In other words this computes `max(x, y, ..)`.

        max_element : function(LuaIVec2):(integer)

        --Returns a vector mask containing the result of a `==` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
        --
        --elements.

        cmpeq : function(LuaIVec2,LuaIVec2):(LuaBVec2)

        --Returns a vector mask containing the result of a `!=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
        --
        --elements.

        cmpne : function(LuaIVec2,LuaIVec2):(LuaBVec2)

        --Returns a vector mask containing the result of a `>=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
        --
        --elements.

        cmpge : function(LuaIVec2,LuaIVec2):(LuaBVec2)

        --Returns a vector mask containing the result of a `>` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
        --
        --elements.

        cmpgt : function(LuaIVec2,LuaIVec2):(LuaBVec2)

        --Returns a vector mask containing the result of a `<=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
        --
        --elements.

        cmple : function(LuaIVec2,LuaIVec2):(LuaBVec2)

        --Returns a vector mask containing the result of a `<` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
        --
        --elements.

        cmplt : function(LuaIVec2,LuaIVec2):(LuaBVec2)

        --Returns a vector containing the absolute value of each element of `self`.

        abs : function(LuaIVec2):(LuaIVec2)

        --Returns a vector with elements representing the sign of `self`.
        --
        --
        --
        -- - `0` if the number is zero
        --
        -- - `1` if the number is positive
        --
        -- - `-1` if the number is negative

        signum : function(LuaIVec2):(LuaIVec2)

        --Returns a bitmask with the lowest 2 bits set to the sign bits from the elements of `self`.
        --
        --
        --
        --A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
        --
        --into the first lowest bit, element `y` into the second, etc.

        is_negative_bitmask : function(LuaIVec2):(integer)

        --Computes the squared length of `self`.

        length_squared : function(LuaIVec2):(integer)

        --Compute the squared euclidean distance between two points in space.

        distance_squared : function(LuaIVec2,LuaIVec2):(integer)

        --Returns a vector that is equal to `self` rotated by 90 degrees.

        perp : function(LuaIVec2):(LuaIVec2)

        --The perpendicular dot product of `self` and `rhs`.
        --
        --Also known as the wedge product, 2D cross product, and determinant.

        perp_dot : function(LuaIVec2,LuaIVec2):(integer)

        --Returns `rhs` rotated by the angle of `self`. If `self` is normalized,
        --
        --then this just rotation. This is what you usually want. Otherwise,
        --
        --it will be like a rotation with a multiplication by `self`'s length.

        rotate : function(LuaIVec2,LuaIVec2):(LuaIVec2)

        --Casts all elements of `self` to `f32`.

        as_vec2 : function(LuaIVec2):(LuaVec2)

        --Casts all elements of `self` to `f64`.

        as_dvec2 : function(LuaIVec2):(LuaDVec2)

        --Casts all elements of `self` to `u32`.

        as_uvec2 : function(LuaIVec2):(LuaUVec2)

        --Creates a new vector.

        new : function((integer),(integer)):(LuaIVec2)

        --Creates a vector with all elements set to `v`.

        splat : function(integer):(LuaIVec2)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec2),(LuaIVec2),(LuaIVec2)):(LuaIVec2)

        __tostring : function(LuaIVec2):(string)

        __unm : function(LuaIVec2):(LuaIVec2)

        __index : function(LuaIVec2,integer):(integer)

        __newindex : function(LuaIVec2,(integer),(integer)):()

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaIVec2Class

        userdata
    
    --Collection of static methods for [`LuaIVec2`].

        --Creates a new vector.

        new : function((integer),(integer)):(LuaIVec2)

        --Creates a vector with all elements set to `v`.

        splat : function(integer):(LuaIVec2)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec2),(LuaIVec2),(LuaIVec2)):(LuaIVec2)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaIVec3

        userdata
    
    --

        x : integer

        y : integer

        z : integer

        --Creates a 4D vector from `self` and the given `w` value.

        extend : function(LuaIVec3,integer):(LuaIVec4)

        --Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
        --
        --
        --
        --Truncation may also be performed by using [`self.xy()`][crate::swizzles::Vec3Swizzles::xy()].

        truncate : function(LuaIVec3):(LuaIVec2)

        --Computes the dot product of `self` and `rhs`.

        dot : function(LuaIVec3,LuaIVec3):(integer)

        --Returns a vector where every component is the dot product of `self` and `rhs`.

        dot_into_vec : function(LuaIVec3,LuaIVec3):(LuaIVec3)

        --Computes the cross product of `self` and `rhs`.

        cross : function(LuaIVec3,LuaIVec3):(LuaIVec3)

        --Returns a vector containing the minimum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

        min : function(LuaIVec3,LuaIVec3):(LuaIVec3)

        --Returns a vector containing the maximum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

        max : function(LuaIVec3,LuaIVec3):(LuaIVec3)

        --Component-wise clamping of values, similar to [`i32::clamp`].
        --
        --
        --
        --Each element in `min` must be less-or-equal to the corresponding element in `max`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `min` is greater than `max` when `glam_assert` is enabled.

        clamp : function(LuaIVec3,(LuaIVec3),(LuaIVec3)):(LuaIVec3)

        --Returns the horizontal minimum of `self`.
        --
        --
        --
        --In other words this computes `min(x, y, ..)`.

        min_element : function(LuaIVec3):(integer)

        --Returns the horizontal maximum of `self`.
        --
        --
        --
        --In other words this computes `max(x, y, ..)`.

        max_element : function(LuaIVec3):(integer)

        --Returns a vector mask containing the result of a `==` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
        --
        --elements.

        cmpeq : function(LuaIVec3,LuaIVec3):(LuaBVec3)

        --Returns a vector mask containing the result of a `!=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
        --
        --elements.

        cmpne : function(LuaIVec3,LuaIVec3):(LuaBVec3)

        --Returns a vector mask containing the result of a `>=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
        --
        --elements.

        cmpge : function(LuaIVec3,LuaIVec3):(LuaBVec3)

        --Returns a vector mask containing the result of a `>` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
        --
        --elements.

        cmpgt : function(LuaIVec3,LuaIVec3):(LuaBVec3)

        --Returns a vector mask containing the result of a `<=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
        --
        --elements.

        cmple : function(LuaIVec3,LuaIVec3):(LuaBVec3)

        --Returns a vector mask containing the result of a `<` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
        --
        --elements.

        cmplt : function(LuaIVec3,LuaIVec3):(LuaBVec3)

        --Returns a vector containing the absolute value of each element of `self`.

        abs : function(LuaIVec3):(LuaIVec3)

        --Returns a vector with elements representing the sign of `self`.
        --
        --
        --
        -- - `0` if the number is zero
        --
        -- - `1` if the number is positive
        --
        -- - `-1` if the number is negative

        signum : function(LuaIVec3):(LuaIVec3)

        --Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of `self`.
        --
        --
        --
        --A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
        --
        --into the first lowest bit, element `y` into the second, etc.

        is_negative_bitmask : function(LuaIVec3):(integer)

        --Computes the squared length of `self`.

        length_squared : function(LuaIVec3):(integer)

        --Compute the squared euclidean distance between two points in space.

        distance_squared : function(LuaIVec3,LuaIVec3):(integer)

        --Casts all elements of `self` to `f32`.

        as_vec3 : function(LuaIVec3):(LuaVec3)

        --Casts all elements of `self` to `f32`.

        as_vec3a : function(LuaIVec3):(LuaVec3A)

        --Casts all elements of `self` to `f64`.

        as_dvec3 : function(LuaIVec3):(LuaDVec3)

        --Casts all elements of `self` to `u32`.

        as_uvec3 : function(LuaIVec3):(LuaUVec3)

        --Creates a new vector.

        new : function((integer),(integer),(integer)):(LuaIVec3)

        --Creates a vector with all elements set to `v`.

        splat : function(integer):(LuaIVec3)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec3),(LuaIVec3),(LuaIVec3)):(LuaIVec3)

        __tostring : function(LuaIVec3):(string)

        __unm : function(LuaIVec3):(LuaIVec3)

        __index : function(LuaIVec3,integer):(integer)

        __newindex : function(LuaIVec3,(integer),(integer)):()

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaIVec3Class

        userdata
    
    --Collection of static methods for [`LuaIVec3`].

        --Creates a new vector.

        new : function((integer),(integer),(integer)):(LuaIVec3)

        --Creates a vector with all elements set to `v`.

        splat : function(integer):(LuaIVec3)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec3),(LuaIVec3),(LuaIVec3)):(LuaIVec3)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaIVec4

        userdata
    
    --

        x : integer

        y : integer

        z : integer

        w : integer

        --Creates a 2D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
        --
        --
        --
        --Truncation to [`IVec3`] may also be performed by using [`self.xyz()`][crate::swizzles::Vec4Swizzles::xyz()].

        truncate : function(LuaIVec4):(LuaIVec3)

        --Computes the dot product of `self` and `rhs`.

        dot : function(LuaIVec4,LuaIVec4):(integer)

        --Returns a vector where every component is the dot product of `self` and `rhs`.

        dot_into_vec : function(LuaIVec4,LuaIVec4):(LuaIVec4)

        --Returns a vector containing the minimum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

        min : function(LuaIVec4,LuaIVec4):(LuaIVec4)

        --Returns a vector containing the maximum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

        max : function(LuaIVec4,LuaIVec4):(LuaIVec4)

        --Component-wise clamping of values, similar to [`i32::clamp`].
        --
        --
        --
        --Each element in `min` must be less-or-equal to the corresponding element in `max`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `min` is greater than `max` when `glam_assert` is enabled.

        clamp : function(LuaIVec4,(LuaIVec4),(LuaIVec4)):(LuaIVec4)

        --Returns the horizontal minimum of `self`.
        --
        --
        --
        --In other words this computes `min(x, y, ..)`.

        min_element : function(LuaIVec4):(integer)

        --Returns the horizontal maximum of `self`.
        --
        --
        --
        --In other words this computes `max(x, y, ..)`.

        max_element : function(LuaIVec4):(integer)

        --Returns a vector mask containing the result of a `==` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
        --
        --elements.

        cmpeq : function(LuaIVec4,LuaIVec4):(LuaBVec4)

        --Returns a vector mask containing the result of a `!=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
        --
        --elements.

        cmpne : function(LuaIVec4,LuaIVec4):(LuaBVec4)

        --Returns a vector mask containing the result of a `>=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
        --
        --elements.

        cmpge : function(LuaIVec4,LuaIVec4):(LuaBVec4)

        --Returns a vector mask containing the result of a `>` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
        --
        --elements.

        cmpgt : function(LuaIVec4,LuaIVec4):(LuaBVec4)

        --Returns a vector mask containing the result of a `<=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
        --
        --elements.

        cmple : function(LuaIVec4,LuaIVec4):(LuaBVec4)

        --Returns a vector mask containing the result of a `<` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
        --
        --elements.

        cmplt : function(LuaIVec4,LuaIVec4):(LuaBVec4)

        --Returns a vector containing the absolute value of each element of `self`.

        abs : function(LuaIVec4):(LuaIVec4)

        --Returns a vector with elements representing the sign of `self`.
        --
        --
        --
        -- - `0` if the number is zero
        --
        -- - `1` if the number is positive
        --
        -- - `-1` if the number is negative

        signum : function(LuaIVec4):(LuaIVec4)

        --Returns a bitmask with the lowest 4 bits set to the sign bits from the elements of `self`.
        --
        --
        --
        --A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
        --
        --into the first lowest bit, element `y` into the second, etc.

        is_negative_bitmask : function(LuaIVec4):(integer)

        --Computes the squared length of `self`.

        length_squared : function(LuaIVec4):(integer)

        --Compute the squared euclidean distance between two points in space.

        distance_squared : function(LuaIVec4,LuaIVec4):(integer)

        --Casts all elements of `self` to `f32`.

        as_vec4 : function(LuaIVec4):(LuaVec4)

        --Casts all elements of `self` to `f64`.

        as_dvec4 : function(LuaIVec4):(LuaDVec4)

        --Casts all elements of `self` to `u32`.

        as_uvec4 : function(LuaIVec4):(LuaUVec4)

        --Creates a new vector.

        new : function((integer),(integer),(integer),(integer)):(LuaIVec4)

        --Creates a vector with all elements set to `v`.

        splat : function(integer):(LuaIVec4)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec4),(LuaIVec4),(LuaIVec4)):(LuaIVec4)

        __tostring : function(LuaIVec4):(string)

        __unm : function(LuaIVec4):(LuaIVec4)

        __index : function(LuaIVec4,integer):(integer)

        __newindex : function(LuaIVec4,(integer),(integer)):()

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaIVec4Class

        userdata
    
    --Collection of static methods for [`LuaIVec4`].

        --Creates a new vector.

        new : function((integer),(integer),(integer),(integer)):(LuaIVec4)

        --Creates a vector with all elements set to `v`.

        splat : function(integer):(LuaIVec4)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec4),(LuaIVec4),(LuaIVec4)):(LuaIVec4)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaUVec2

        userdata
    
    --

        x : integer

        y : integer

        --Creates a 3D vector from `self` and the given `z` value.

        extend : function(LuaUVec2,integer):(LuaUVec3)

        --Computes the dot product of `self` and `rhs`.

        dot : function(LuaUVec2,LuaUVec2):(integer)

        --Returns a vector where every component is the dot product of `self` and `rhs`.

        dot_into_vec : function(LuaUVec2,LuaUVec2):(LuaUVec2)

        --Returns a vector containing the minimum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

        min : function(LuaUVec2,LuaUVec2):(LuaUVec2)

        --Returns a vector containing the maximum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

        max : function(LuaUVec2,LuaUVec2):(LuaUVec2)

        --Component-wise clamping of values, similar to [`u32::clamp`].
        --
        --
        --
        --Each element in `min` must be less-or-equal to the corresponding element in `max`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `min` is greater than `max` when `glam_assert` is enabled.

        clamp : function(LuaUVec2,(LuaUVec2),(LuaUVec2)):(LuaUVec2)

        --Returns the horizontal minimum of `self`.
        --
        --
        --
        --In other words this computes `min(x, y, ..)`.

        min_element : function(LuaUVec2):(integer)

        --Returns the horizontal maximum of `self`.
        --
        --
        --
        --In other words this computes `max(x, y, ..)`.

        max_element : function(LuaUVec2):(integer)

        --Returns a vector mask containing the result of a `==` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
        --
        --elements.

        cmpeq : function(LuaUVec2,LuaUVec2):(LuaBVec2)

        --Returns a vector mask containing the result of a `!=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
        --
        --elements.

        cmpne : function(LuaUVec2,LuaUVec2):(LuaBVec2)

        --Returns a vector mask containing the result of a `>=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
        --
        --elements.

        cmpge : function(LuaUVec2,LuaUVec2):(LuaBVec2)

        --Returns a vector mask containing the result of a `>` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
        --
        --elements.

        cmpgt : function(LuaUVec2,LuaUVec2):(LuaBVec2)

        --Returns a vector mask containing the result of a `<=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
        --
        --elements.

        cmple : function(LuaUVec2,LuaUVec2):(LuaBVec2)

        --Returns a vector mask containing the result of a `<` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
        --
        --elements.

        cmplt : function(LuaUVec2,LuaUVec2):(LuaBVec2)

        --Computes the squared length of `self`.

        length_squared : function(LuaUVec2):(integer)

        --Casts all elements of `self` to `f32`.

        as_vec2 : function(LuaUVec2):(LuaVec2)

        --Casts all elements of `self` to `f64`.

        as_dvec2 : function(LuaUVec2):(LuaDVec2)

        --Casts all elements of `self` to `i32`.

        as_ivec2 : function(LuaUVec2):(LuaIVec2)

        --Creates a new vector.

        new : function((integer),(integer)):(LuaUVec2)

        --Creates a vector with all elements set to `v`.

        splat : function(integer):(LuaUVec2)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec2),(LuaUVec2),(LuaUVec2)):(LuaUVec2)

        __tostring : function(LuaUVec2):(string)

        __index : function(LuaUVec2,integer):(integer)

        __newindex : function(LuaUVec2,(integer),(integer)):()

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaUVec2Class

        userdata
    
    --Collection of static methods for [`LuaUVec2`].

        --Creates a new vector.

        new : function((integer),(integer)):(LuaUVec2)

        --Creates a vector with all elements set to `v`.

        splat : function(integer):(LuaUVec2)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec2),(LuaUVec2),(LuaUVec2)):(LuaUVec2)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaUVec3

        userdata
    
    --

        x : integer

        y : integer

        z : integer

        --Creates a 4D vector from `self` and the given `w` value.

        extend : function(LuaUVec3,integer):(LuaUVec4)

        --Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
        --
        --
        --
        --Truncation may also be performed by using [`self.xy()`][crate::swizzles::Vec3Swizzles::xy()].

        truncate : function(LuaUVec3):(LuaUVec2)

        --Computes the dot product of `self` and `rhs`.

        dot : function(LuaUVec3,LuaUVec3):(integer)

        --Returns a vector where every component is the dot product of `self` and `rhs`.

        dot_into_vec : function(LuaUVec3,LuaUVec3):(LuaUVec3)

        --Computes the cross product of `self` and `rhs`.

        cross : function(LuaUVec3,LuaUVec3):(LuaUVec3)

        --Returns a vector containing the minimum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

        min : function(LuaUVec3,LuaUVec3):(LuaUVec3)

        --Returns a vector containing the maximum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

        max : function(LuaUVec3,LuaUVec3):(LuaUVec3)

        --Component-wise clamping of values, similar to [`u32::clamp`].
        --
        --
        --
        --Each element in `min` must be less-or-equal to the corresponding element in `max`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `min` is greater than `max` when `glam_assert` is enabled.

        clamp : function(LuaUVec3,(LuaUVec3),(LuaUVec3)):(LuaUVec3)

        --Returns the horizontal minimum of `self`.
        --
        --
        --
        --In other words this computes `min(x, y, ..)`.

        min_element : function(LuaUVec3):(integer)

        --Returns the horizontal maximum of `self`.
        --
        --
        --
        --In other words this computes `max(x, y, ..)`.

        max_element : function(LuaUVec3):(integer)

        --Returns a vector mask containing the result of a `==` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
        --
        --elements.

        cmpeq : function(LuaUVec3,LuaUVec3):(LuaBVec3)

        --Returns a vector mask containing the result of a `!=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
        --
        --elements.

        cmpne : function(LuaUVec3,LuaUVec3):(LuaBVec3)

        --Returns a vector mask containing the result of a `>=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
        --
        --elements.

        cmpge : function(LuaUVec3,LuaUVec3):(LuaBVec3)

        --Returns a vector mask containing the result of a `>` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
        --
        --elements.

        cmpgt : function(LuaUVec3,LuaUVec3):(LuaBVec3)

        --Returns a vector mask containing the result of a `<=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
        --
        --elements.

        cmple : function(LuaUVec3,LuaUVec3):(LuaBVec3)

        --Returns a vector mask containing the result of a `<` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
        --
        --elements.

        cmplt : function(LuaUVec3,LuaUVec3):(LuaBVec3)

        --Computes the squared length of `self`.

        length_squared : function(LuaUVec3):(integer)

        --Casts all elements of `self` to `f32`.

        as_vec3 : function(LuaUVec3):(LuaVec3)

        --Casts all elements of `self` to `f32`.

        as_vec3a : function(LuaUVec3):(LuaVec3A)

        --Casts all elements of `self` to `f64`.

        as_dvec3 : function(LuaUVec3):(LuaDVec3)

        --Casts all elements of `self` to `i32`.

        as_ivec3 : function(LuaUVec3):(LuaIVec3)

        --Creates a new vector.

        new : function((integer),(integer),(integer)):(LuaUVec3)

        --Creates a vector with all elements set to `v`.

        splat : function(integer):(LuaUVec3)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec3),(LuaUVec3),(LuaUVec3)):(LuaUVec3)

        __tostring : function(LuaUVec3):(string)

        __index : function(LuaUVec3,integer):(integer)

        __newindex : function(LuaUVec3,(integer),(integer)):()

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaUVec3Class

        userdata
    
    --Collection of static methods for [`LuaUVec3`].

        --Creates a new vector.

        new : function((integer),(integer),(integer)):(LuaUVec3)

        --Creates a vector with all elements set to `v`.

        splat : function(integer):(LuaUVec3)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec3),(LuaUVec3),(LuaUVec3)):(LuaUVec3)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaUVec4

        userdata
    
    --

        x : integer

        y : integer

        z : integer

        w : integer

        --Creates a 2D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
        --
        --
        --
        --Truncation to [`UVec3`] may also be performed by using [`self.xyz()`][crate::swizzles::Vec4Swizzles::xyz()].

        truncate : function(LuaUVec4):(LuaUVec3)

        --Computes the dot product of `self` and `rhs`.

        dot : function(LuaUVec4,LuaUVec4):(integer)

        --Returns a vector where every component is the dot product of `self` and `rhs`.

        dot_into_vec : function(LuaUVec4,LuaUVec4):(LuaUVec4)

        --Returns a vector containing the minimum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

        min : function(LuaUVec4,LuaUVec4):(LuaUVec4)

        --Returns a vector containing the maximum values for each element of `self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

        max : function(LuaUVec4,LuaUVec4):(LuaUVec4)

        --Component-wise clamping of values, similar to [`u32::clamp`].
        --
        --
        --
        --Each element in `min` must be less-or-equal to the corresponding element in `max`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `min` is greater than `max` when `glam_assert` is enabled.

        clamp : function(LuaUVec4,(LuaUVec4),(LuaUVec4)):(LuaUVec4)

        --Returns the horizontal minimum of `self`.
        --
        --
        --
        --In other words this computes `min(x, y, ..)`.

        min_element : function(LuaUVec4):(integer)

        --Returns the horizontal maximum of `self`.
        --
        --
        --
        --In other words this computes `max(x, y, ..)`.

        max_element : function(LuaUVec4):(integer)

        --Returns a vector mask containing the result of a `==` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
        --
        --elements.

        cmpeq : function(LuaUVec4,LuaUVec4):(LuaBVec4)

        --Returns a vector mask containing the result of a `!=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
        --
        --elements.

        cmpne : function(LuaUVec4,LuaUVec4):(LuaBVec4)

        --Returns a vector mask containing the result of a `>=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
        --
        --elements.

        cmpge : function(LuaUVec4,LuaUVec4):(LuaBVec4)

        --Returns a vector mask containing the result of a `>` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
        --
        --elements.

        cmpgt : function(LuaUVec4,LuaUVec4):(LuaBVec4)

        --Returns a vector mask containing the result of a `<=` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
        --
        --elements.

        cmple : function(LuaUVec4,LuaUVec4):(LuaBVec4)

        --Returns a vector mask containing the result of a `<` comparison for each element of
        --
        --`self` and `rhs`.
        --
        --
        --
        --In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
        --
        --elements.

        cmplt : function(LuaUVec4,LuaUVec4):(LuaBVec4)

        --Computes the squared length of `self`.

        length_squared : function(LuaUVec4):(integer)

        --Casts all elements of `self` to `f32`.

        as_vec4 : function(LuaUVec4):(LuaVec4)

        --Casts all elements of `self` to `f64`.

        as_dvec4 : function(LuaUVec4):(LuaDVec4)

        --Casts all elements of `self` to `i32`.

        as_ivec4 : function(LuaUVec4):(LuaIVec4)

        --Creates a new vector.

        new : function((integer),(integer),(integer),(integer)):(LuaUVec4)

        --Creates a vector with all elements set to `v`.

        splat : function(integer):(LuaUVec4)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec4),(LuaUVec4),(LuaUVec4)):(LuaUVec4)

        __tostring : function(LuaUVec4):(string)

        __index : function(LuaUVec4,integer):(integer)

        __newindex : function(LuaUVec4,(integer),(integer)):()

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaUVec4Class

        userdata
    
    --Collection of static methods for [`LuaUVec4`].

        --Creates a new vector.

        new : function((integer),(integer),(integer),(integer)):(LuaUVec4)

        --Creates a vector with all elements set to `v`.

        splat : function(integer):(LuaUVec4)

        --Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        --
        --for each element of `self`.
        --
        --
        --
        --A true element in the mask uses the corresponding element from `if_true`, and false
        --
        --uses the element from `if_false`.

        select : function((LuaBVec4),(LuaUVec4),(LuaUVec4)):(LuaUVec4)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

        __mod : function((any),(any)):(any)

    end
    
    
    record LuaMat3

        userdata
    
    --

        x_axis : LuaVec3

        y_axis : LuaVec3

        z_axis : LuaVec3

        --Returns the matrix column for the given `index`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Panics if `index` is greater than 2.

        col : function(LuaMat3,integer):(LuaVec3)

        --Returns the matrix row for the given `index`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Panics if `index` is greater than 2.

        row : function(LuaMat3,integer):(LuaVec3)

        --Returns `true` if, and only if, all elements are finite.
        --
        --If any element is either `NaN`, positive or negative infinity, this will return `false`.

        is_finite : function(LuaMat3):(boolean)

        --Returns `true` if any elements are `NaN`.

        is_nan : function(LuaMat3):(boolean)

        --Returns the transpose of `self`.

        transpose : function(LuaMat3):(LuaMat3)

        --Returns the determinant of `self`.

        determinant : function(LuaMat3):(number)

        --Returns the inverse of `self`.
        --
        --
        --
        --If the matrix is not invertible the returned matrix will be invalid.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if the determinant of `self` is zero when `glam_assert` is enabled.

        inverse : function(LuaMat3):(LuaMat3)

        --Transforms the given 2D vector as a point.
        --
        --
        --
        --This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `1`.
        --
        --
        --
        --This method assumes that `self` contains a valid affine transform.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.

        transform_point2 : function(LuaMat3,LuaVec2):(LuaVec2)

        --Rotates the given 2D vector.
        --
        --
        --
        --This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `0`.
        --
        --
        --
        --This method assumes that `self` contains a valid affine transform.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.

        transform_vector2 : function(LuaMat3,LuaVec2):(LuaVec2)

        --Transforms a 3D vector.

        mul_vec3 : function(LuaMat3,LuaVec3):(LuaVec3)

        --Transforms a [`Vec3A`].

        mul_vec3a : function(LuaMat3,LuaVec3A):(LuaVec3A)

        --Multiplies two 3x3 matrices.

        mul_mat3 : function(LuaMat3,LuaMat3):(LuaMat3)

        --Adds two 3x3 matrices.

        add_mat3 : function(LuaMat3,LuaMat3):(LuaMat3)

        --Subtracts two 3x3 matrices.

        sub_mat3 : function(LuaMat3,LuaMat3):(LuaMat3)

        --Multiplies a 3x3 matrix by a scalar.

        mul_scalar : function(LuaMat3,number):(LuaMat3)

        --Returns true if the absolute difference of all elements between `self` and `rhs`
        --
        --is less than or equal to `max_abs_diff`.
        --
        --
        --
        --This can be used to compare if two matrices contain similar elements. It works best
        --
        --when comparing with a known value. The `max_abs_diff` that should be used used
        --
        --depends on the values being compared against.
        --
        --
        --
        --For more see
        --
        --[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

        abs_diff_eq : function(LuaMat3,(LuaMat3),(number)):(boolean)

        as_dmat3 : function(LuaMat3):(LuaDMat3)

        --Creates a 3x3 matrix from three column vectors.

        from_cols : function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaMat3)

        --Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.

        from_diagonal : function(LuaVec3):(LuaMat3)

        --Creates a 3x3 matrix from a 4x4 matrix, discarding the 4th row and column.

        from_mat4 : function(LuaMat4):(LuaMat3)

        --Creates a 3D rotation matrix from the given quaternion.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rotation` is not normalized when `glam_assert` is enabled.

        from_quat : function(LuaQuat):(LuaMat3)

        --Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
        --
        --radians).
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `axis` is not normalized when `glam_assert` is enabled.

        from_axis_angle : function((LuaVec3),(number)):(LuaMat3)

        --Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
        --
        --radians).

        from_euler : function((LuaEulerRot),(number),(number),(number)):(LuaMat3)

        --Creates a 3D rotation matrix from `angle` (in radians) around the x axis.

        from_rotation_x : function(number):(LuaMat3)

        --Creates a 3D rotation matrix from `angle` (in radians) around the y axis.

        from_rotation_y : function(number):(LuaMat3)

        --Creates a 3D rotation matrix from `angle` (in radians) around the z axis.

        from_rotation_z : function(number):(LuaMat3)

        --Creates an affine transformation matrix from the given 2D `translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_translation : function(LuaVec2):(LuaMat3)

        --Creates an affine transformation matrix from the given 2D rotation `angle` (in
        --
        --radians).
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_angle : function(number):(LuaMat3)

        --Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
        --
        --radians) and `translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_scale_angle_translation : function((LuaVec2),(number),(LuaVec2)):(LuaMat3)

        --Creates an affine transformation matrix from the given non-uniform 2D `scale`.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if all elements of `scale` are zero when `glam_assert` is enabled.

        from_scale : function(LuaVec2):(LuaMat3)

        --Creates an affine transformation matrix from the given 2x2 matrix.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_mat2 : function(LuaMat2):(LuaMat3)

        __tostring : function(LuaMat3):(string)

        __unm : function(LuaMat3):(LuaMat3)

        __index : function(LuaMat3,integer):(LuaVec3)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaMat3Class

        userdata
    
    --Collection of static methods for [`LuaMat3`].

        --Creates a 3x3 matrix from three column vectors.

        from_cols : function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaMat3)

        --Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.

        from_diagonal : function(LuaVec3):(LuaMat3)

        --Creates a 3x3 matrix from a 4x4 matrix, discarding the 4th row and column.

        from_mat4 : function(LuaMat4):(LuaMat3)

        --Creates a 3D rotation matrix from the given quaternion.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rotation` is not normalized when `glam_assert` is enabled.

        from_quat : function(LuaQuat):(LuaMat3)

        --Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
        --
        --radians).
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `axis` is not normalized when `glam_assert` is enabled.

        from_axis_angle : function((LuaVec3),(number)):(LuaMat3)

        --Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
        --
        --radians).

        from_euler : function((LuaEulerRot),(number),(number),(number)):(LuaMat3)

        --Creates a 3D rotation matrix from `angle` (in radians) around the x axis.

        from_rotation_x : function(number):(LuaMat3)

        --Creates a 3D rotation matrix from `angle` (in radians) around the y axis.

        from_rotation_y : function(number):(LuaMat3)

        --Creates a 3D rotation matrix from `angle` (in radians) around the z axis.

        from_rotation_z : function(number):(LuaMat3)

        --Creates an affine transformation matrix from the given 2D `translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_translation : function(LuaVec2):(LuaMat3)

        --Creates an affine transformation matrix from the given 2D rotation `angle` (in
        --
        --radians).
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_angle : function(number):(LuaMat3)

        --Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
        --
        --radians) and `translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_scale_angle_translation : function((LuaVec2),(number),(LuaVec2)):(LuaMat3)

        --Creates an affine transformation matrix from the given non-uniform 2D `scale`.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if all elements of `scale` are zero when `glam_assert` is enabled.

        from_scale : function(LuaVec2):(LuaMat3)

        --Creates an affine transformation matrix from the given 2x2 matrix.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_mat2 : function(LuaMat2):(LuaMat3)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaMat2

        userdata
    
    --

        --Returns the matrix column for the given `index`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Panics if `index` is greater than 1.

        col : function(LuaMat2,integer):(LuaVec2)

        --Returns the matrix row for the given `index`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Panics if `index` is greater than 1.

        row : function(LuaMat2,integer):(LuaVec2)

        --Returns `true` if, and only if, all elements are finite.
        --
        --If any element is either `NaN`, positive or negative infinity, this will return `false`.

        is_finite : function(LuaMat2):(boolean)

        --Returns `true` if any elements are `NaN`.

        is_nan : function(LuaMat2):(boolean)

        --Returns the transpose of `self`.

        transpose : function(LuaMat2):(LuaMat2)

        --Returns the determinant of `self`.

        determinant : function(LuaMat2):(number)

        --Returns the inverse of `self`.
        --
        --
        --
        --If the matrix is not invertible the returned matrix will be invalid.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if the determinant of `self` is zero when `glam_assert` is enabled.

        inverse : function(LuaMat2):(LuaMat2)

        --Transforms a 2D vector.

        mul_vec2 : function(LuaMat2,LuaVec2):(LuaVec2)

        --Multiplies two 2x2 matrices.

        mul_mat2 : function(LuaMat2,LuaMat2):(LuaMat2)

        --Adds two 2x2 matrices.

        add_mat2 : function(LuaMat2,LuaMat2):(LuaMat2)

        --Subtracts two 2x2 matrices.

        sub_mat2 : function(LuaMat2,LuaMat2):(LuaMat2)

        --Multiplies a 2x2 matrix by a scalar.

        mul_scalar : function(LuaMat2,number):(LuaMat2)

        --Returns true if the absolute difference of all elements between `self` and `rhs`
        --
        --is less than or equal to `max_abs_diff`.
        --
        --
        --
        --This can be used to compare if two matrices contain similar elements. It works best
        --
        --when comparing with a known value. The `max_abs_diff` that should be used used
        --
        --depends on the values being compared against.
        --
        --
        --
        --For more see
        --
        --[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

        abs_diff_eq : function(LuaMat2,(LuaMat2),(number)):(boolean)

        as_dmat2 : function(LuaMat2):(LuaDMat2)

        --Creates a 2x2 matrix from two column vectors.

        from_cols : function((LuaVec2),(LuaVec2)):(LuaMat2)

        --Creates a 2x2 matrix with its diagonal set to `diagonal` and all other entries set to 0.

        from_diagonal : function(LuaVec2):(LuaMat2)

        --Creates a 2x2 matrix containing the combining non-uniform `scale` and rotation of
        --
        --`angle` (in radians).

        from_scale_angle : function((LuaVec2),(number)):(LuaMat2)

        --Creates a 2x2 matrix containing a rotation of `angle` (in radians).

        from_angle : function(number):(LuaMat2)

        --Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.

        from_mat3 : function(LuaMat3):(LuaMat2)

        --Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.

        from_mat3a : function(LuaMat3A):(LuaMat2)

        __tostring : function(LuaMat2):(string)

        __unm : function(LuaMat2):(LuaMat2)

        __index : function(LuaMat2,integer):(LuaVec2)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaMat2Class

        userdata
    
    --Collection of static methods for [`LuaMat2`].

        --Creates a 2x2 matrix from two column vectors.

        from_cols : function((LuaVec2),(LuaVec2)):(LuaMat2)

        --Creates a 2x2 matrix with its diagonal set to `diagonal` and all other entries set to 0.

        from_diagonal : function(LuaVec2):(LuaMat2)

        --Creates a 2x2 matrix containing the combining non-uniform `scale` and rotation of
        --
        --`angle` (in radians).

        from_scale_angle : function((LuaVec2),(number)):(LuaMat2)

        --Creates a 2x2 matrix containing a rotation of `angle` (in radians).

        from_angle : function(number):(LuaMat2)

        --Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.

        from_mat3 : function(LuaMat3):(LuaMat2)

        --Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.

        from_mat3a : function(LuaMat3A):(LuaMat2)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaMat3A

        userdata
    
    --

        x_axis : LuaVec3A

        y_axis : LuaVec3A

        z_axis : LuaVec3A

        --Returns the matrix column for the given `index`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Panics if `index` is greater than 2.

        col : function(LuaMat3A,integer):(LuaVec3A)

        --Returns the matrix row for the given `index`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Panics if `index` is greater than 2.

        row : function(LuaMat3A,integer):(LuaVec3A)

        --Returns `true` if, and only if, all elements are finite.
        --
        --If any element is either `NaN`, positive or negative infinity, this will return `false`.

        is_finite : function(LuaMat3A):(boolean)

        --Returns `true` if any elements are `NaN`.

        is_nan : function(LuaMat3A):(boolean)

        --Returns the transpose of `self`.

        transpose : function(LuaMat3A):(LuaMat3A)

        --Returns the determinant of `self`.

        determinant : function(LuaMat3A):(number)

        --Returns the inverse of `self`.
        --
        --
        --
        --If the matrix is not invertible the returned matrix will be invalid.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if the determinant of `self` is zero when `glam_assert` is enabled.

        inverse : function(LuaMat3A):(LuaMat3A)

        --Transforms the given 2D vector as a point.
        --
        --
        --
        --This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `1`.
        --
        --
        --
        --This method assumes that `self` contains a valid affine transform.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.

        transform_point2 : function(LuaMat3A,LuaVec2):(LuaVec2)

        --Rotates the given 2D vector.
        --
        --
        --
        --This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `0`.
        --
        --
        --
        --This method assumes that `self` contains a valid affine transform.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.

        transform_vector2 : function(LuaMat3A,LuaVec2):(LuaVec2)

        --Transforms a 3D vector.

        mul_vec3 : function(LuaMat3A,LuaVec3):(LuaVec3)

        --Transforms a [`Vec3A`].

        mul_vec3a : function(LuaMat3A,LuaVec3A):(LuaVec3A)

        --Multiplies two 3x3 matrices.

        mul_mat3 : function(LuaMat3A,LuaMat3A):(LuaMat3A)

        --Adds two 3x3 matrices.

        add_mat3 : function(LuaMat3A,LuaMat3A):(LuaMat3A)

        --Subtracts two 3x3 matrices.

        sub_mat3 : function(LuaMat3A,LuaMat3A):(LuaMat3A)

        --Multiplies a 3x3 matrix by a scalar.

        mul_scalar : function(LuaMat3A,number):(LuaMat3A)

        --Returns true if the absolute difference of all elements between `self` and `rhs`
        --
        --is less than or equal to `max_abs_diff`.
        --
        --
        --
        --This can be used to compare if two matrices contain similar elements. It works best
        --
        --when comparing with a known value. The `max_abs_diff` that should be used used
        --
        --depends on the values being compared against.
        --
        --
        --
        --For more see
        --
        --[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

        abs_diff_eq : function(LuaMat3A,(LuaMat3A),(number)):(boolean)

        as_dmat3 : function(LuaMat3A):(LuaDMat3)

        --Creates a 3x3 matrix from three column vectors.

        from_cols : function((LuaVec3A),(LuaVec3A),(LuaVec3A)):(LuaMat3A)

        --Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.

        from_diagonal : function(LuaVec3):(LuaMat3A)

        --Creates a 3x3 matrix from a 4x4 matrix, discarding the 4th row and column.

        from_mat4 : function(LuaMat4):(LuaMat3A)

        --Creates a 3D rotation matrix from the given quaternion.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rotation` is not normalized when `glam_assert` is enabled.

        from_quat : function(LuaQuat):(LuaMat3A)

        --Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
        --
        --radians).
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `axis` is not normalized when `glam_assert` is enabled.

        from_axis_angle : function((LuaVec3),(number)):(LuaMat3A)

        --Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
        --
        --radians).

        from_euler : function((LuaEulerRot),(number),(number),(number)):(LuaMat3A)

        --Creates a 3D rotation matrix from `angle` (in radians) around the x axis.

        from_rotation_x : function(number):(LuaMat3A)

        --Creates a 3D rotation matrix from `angle` (in radians) around the y axis.

        from_rotation_y : function(number):(LuaMat3A)

        --Creates a 3D rotation matrix from `angle` (in radians) around the z axis.

        from_rotation_z : function(number):(LuaMat3A)

        --Creates an affine transformation matrix from the given 2D `translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_translation : function(LuaVec2):(LuaMat3A)

        --Creates an affine transformation matrix from the given 2D rotation `angle` (in
        --
        --radians).
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_angle : function(number):(LuaMat3A)

        --Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
        --
        --radians) and `translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_scale_angle_translation : function((LuaVec2),(number),(LuaVec2)):(LuaMat3A)

        --Creates an affine transformation matrix from the given non-uniform 2D `scale`.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if all elements of `scale` are zero when `glam_assert` is enabled.

        from_scale : function(LuaVec2):(LuaMat3A)

        --Creates an affine transformation matrix from the given 2x2 matrix.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_mat2 : function(LuaMat2):(LuaMat3A)

        __tostring : function(LuaMat3A):(string)

        __unm : function(LuaMat3A):(LuaMat3A)

        __index : function(LuaMat3A,integer):(LuaVec3A)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaMat3AClass

        userdata
    
    --Collection of static methods for [`LuaMat3A`].

        --Creates a 3x3 matrix from three column vectors.

        from_cols : function((LuaVec3A),(LuaVec3A),(LuaVec3A)):(LuaMat3A)

        --Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.

        from_diagonal : function(LuaVec3):(LuaMat3A)

        --Creates a 3x3 matrix from a 4x4 matrix, discarding the 4th row and column.

        from_mat4 : function(LuaMat4):(LuaMat3A)

        --Creates a 3D rotation matrix from the given quaternion.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rotation` is not normalized when `glam_assert` is enabled.

        from_quat : function(LuaQuat):(LuaMat3A)

        --Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
        --
        --radians).
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `axis` is not normalized when `glam_assert` is enabled.

        from_axis_angle : function((LuaVec3),(number)):(LuaMat3A)

        --Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
        --
        --radians).

        from_euler : function((LuaEulerRot),(number),(number),(number)):(LuaMat3A)

        --Creates a 3D rotation matrix from `angle` (in radians) around the x axis.

        from_rotation_x : function(number):(LuaMat3A)

        --Creates a 3D rotation matrix from `angle` (in radians) around the y axis.

        from_rotation_y : function(number):(LuaMat3A)

        --Creates a 3D rotation matrix from `angle` (in radians) around the z axis.

        from_rotation_z : function(number):(LuaMat3A)

        --Creates an affine transformation matrix from the given 2D `translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_translation : function(LuaVec2):(LuaMat3A)

        --Creates an affine transformation matrix from the given 2D rotation `angle` (in
        --
        --radians).
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_angle : function(number):(LuaMat3A)

        --Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
        --
        --radians) and `translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_scale_angle_translation : function((LuaVec2),(number),(LuaVec2)):(LuaMat3A)

        --Creates an affine transformation matrix from the given non-uniform 2D `scale`.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if all elements of `scale` are zero when `glam_assert` is enabled.

        from_scale : function(LuaVec2):(LuaMat3A)

        --Creates an affine transformation matrix from the given 2x2 matrix.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_mat2 : function(LuaMat2):(LuaMat3A)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaMat4

        userdata
    
    --

        x_axis : LuaVec4

        y_axis : LuaVec4

        z_axis : LuaVec4

        w_axis : LuaVec4

        --Returns the matrix column for the given `index`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Panics if `index` is greater than 3.

        col : function(LuaMat4,integer):(LuaVec4)

        --Returns the matrix row for the given `index`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Panics if `index` is greater than 3.

        row : function(LuaMat4,integer):(LuaVec4)

        --Returns `true` if, and only if, all elements are finite.
        --
        --If any element is either `NaN`, positive or negative infinity, this will return `false`.

        is_finite : function(LuaMat4):(boolean)

        --Returns `true` if any elements are `NaN`.

        is_nan : function(LuaMat4):(boolean)

        --Returns the transpose of `self`.

        transpose : function(LuaMat4):(LuaMat4)

        --Returns the determinant of `self`.

        determinant : function(LuaMat4):(number)

        --Returns the inverse of `self`.
        --
        --
        --
        --If the matrix is not invertible the returned matrix will be invalid.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if the determinant of `self` is zero when `glam_assert` is enabled.

        inverse : function(LuaMat4):(LuaMat4)

        --Transforms the given 3D vector as a point, applying perspective correction.
        --
        --
        --
        --This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is `1.0`.
        --
        --The perspective divide is performed meaning the resulting 3D vector is divided by `w`.
        --
        --
        --
        --This method assumes that `self` contains a projective transform.

        project_point3 : function(LuaMat4,LuaVec3):(LuaVec3)

        --Transforms the given 3D vector as a point.
        --
        --
        --
        --This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
        --
        --`1.0`.
        --
        --
        --
        --This method assumes that `self` contains a valid affine transform. It does not perform
        --
        --a persective divide, if `self` contains a perspective transform, or if you are unsure,
        --
        --the [`Self::project_point3()`] method should be used instead.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.

        transform_point3 : function(LuaMat4,LuaVec3):(LuaVec3)

        --Transforms the give 3D vector as a direction.
        --
        --
        --
        --This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
        --
        --`0.0`.
        --
        --
        --
        --This method assumes that `self` contains a valid affine transform.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.

        transform_vector3 : function(LuaMat4,LuaVec3):(LuaVec3)

        --Transforms the given [`Vec3A`] as 3D point.
        --
        --
        --
        --This is the equivalent of multiplying the [`Vec3A`] as a 4D vector where `w` is `1.0`.

        transform_point3a : function(LuaMat4,LuaVec3A):(LuaVec3A)

        --Transforms the give [`Vec3A`] as 3D vector.
        --
        --
        --
        --This is the equivalent of multiplying the [`Vec3A`] as a 4D vector where `w` is `0.0`.

        transform_vector3a : function(LuaMat4,LuaVec3A):(LuaVec3A)

        --Transforms a 4D vector.

        mul_vec4 : function(LuaMat4,LuaVec4):(LuaVec4)

        --Multiplies two 4x4 matrices.

        mul_mat4 : function(LuaMat4,LuaMat4):(LuaMat4)

        --Adds two 4x4 matrices.

        add_mat4 : function(LuaMat4,LuaMat4):(LuaMat4)

        --Subtracts two 4x4 matrices.

        sub_mat4 : function(LuaMat4,LuaMat4):(LuaMat4)

        --Multiplies a 4x4 matrix by a scalar.

        mul_scalar : function(LuaMat4,number):(LuaMat4)

        --Returns true if the absolute difference of all elements between `self` and `rhs`
        --
        --is less than or equal to `max_abs_diff`.
        --
        --
        --
        --This can be used to compare if two matrices contain similar elements. It works best
        --
        --when comparing with a known value. The `max_abs_diff` that should be used used
        --
        --depends on the values being compared against.
        --
        --
        --
        --For more see
        --
        --[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

        abs_diff_eq : function(LuaMat4,(LuaMat4),(number)):(boolean)

        as_dmat4 : function(LuaMat4):(LuaDMat4)

        --Creates a 4x4 matrix from four column vectors.

        from_cols : function((LuaVec4),(LuaVec4),(LuaVec4),(LuaVec4)):(LuaMat4)

        --Creates a 4x4 matrix with its diagonal set to `diagonal` and all other entries set to 0.

        from_diagonal : function(LuaVec4):(LuaMat4)

        --Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
        --
        --`translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rotation` is not normalized when `glam_assert` is enabled.

        from_scale_rotation_translation : function((LuaVec3),(LuaQuat),(LuaVec3)):(LuaMat4)

        --Creates an affine transformation matrix from the given 3D `translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rotation` is not normalized when `glam_assert` is enabled.

        from_rotation_translation : function((LuaQuat),(LuaVec3)):(LuaMat4)

        --Creates an affine transformation matrix from the given `rotation` quaternion.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rotation` is not normalized when `glam_assert` is enabled.

        from_quat : function(LuaQuat):(LuaMat4)

        --Creates an affine transformation matrix from the given 3x3 linear transformation
        --
        --matrix.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_mat3 : function(LuaMat3):(LuaMat4)

        --Creates an affine transformation matrix from the given 3x3 linear transformation
        --
        --matrix.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_mat3a : function(LuaMat3A):(LuaMat4)

        --Creates an affine transformation matrix from the given 3D `translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_translation : function(LuaVec3):(LuaMat4)

        --Creates an affine transformation matrix containing a 3D rotation around a normalized
        --
        --rotation `axis` of `angle` (in radians).
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `axis` is not normalized when `glam_assert` is enabled.

        from_axis_angle : function((LuaVec3),(number)):(LuaMat4)

        --Creates a affine transformation matrix containing a rotation from the given euler
        --
        --rotation sequence and angles (in radians).
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_euler : function((LuaEulerRot),(number),(number),(number)):(LuaMat4)

        --Creates an affine transformation matrix containing a 3D rotation around the x axis of
        --
        --`angle` (in radians).
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_rotation_x : function(number):(LuaMat4)

        --Creates an affine transformation matrix containing a 3D rotation around the y axis of
        --
        --`angle` (in radians).
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_rotation_y : function(number):(LuaMat4)

        --Creates an affine transformation matrix containing a 3D rotation around the z axis of
        --
        --`angle` (in radians).
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_rotation_z : function(number):(LuaMat4)

        --Creates an affine transformation matrix containing the given 3D non-uniform `scale`.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if all elements of `scale` are zero when `glam_assert` is enabled.

        from_scale : function(LuaVec3):(LuaMat4)

        --Creates a left-handed view matrix using a camera position, an up direction, and a facing
        --
        --direction.
        --
        --
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.

        look_to_lh : function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaMat4)

        --Creates a right-handed view matrix using a camera position, an up direction, and a facing
        --
        --direction.
        --
        --
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.

        look_to_rh : function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaMat4)

        --Creates a left-handed view matrix using a camera position, an up direction, and a focal
        --
        --point.
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `up` is not normalized when `glam_assert` is enabled.

        look_at_lh : function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaMat4)

        --Creates a right-handed view matrix using a camera position, an up direction, and a focal
        --
        --point.
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `up` is not normalized when `glam_assert` is enabled.

        look_at_rh : function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaMat4)

        --Creates a right-handed perspective projection matrix with [-1,1] depth range.
        --
        --This is the same as the OpenGL `gluPerspective` function.
        --
        --See <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml>

        perspective_rh_gl : function((number),(number),(number),(number)):(LuaMat4)

        --Creates a left-handed perspective projection matrix with `[0,1]` depth range.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
        --
        --enabled.

        perspective_lh : function((number),(number),(number),(number)):(LuaMat4)

        --Creates a right-handed perspective projection matrix with `[0,1]` depth range.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
        --
        --enabled.

        perspective_rh : function((number),(number),(number),(number)):(LuaMat4)

        --Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.

        perspective_infinite_lh : function((number),(number),(number)):(LuaMat4)

        --Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.

        perspective_infinite_reverse_lh : function((number),(number),(number)):(LuaMat4)

        --Creates an infinite right-handed perspective projection matrix with
        --
        --`[0,1]` depth range.

        perspective_infinite_rh : function((number),(number),(number)):(LuaMat4)

        --Creates an infinite reverse right-handed perspective projection matrix
        --
        --with `[0,1]` depth range.

        perspective_infinite_reverse_rh : function((number),(number),(number)):(LuaMat4)

        --Creates a right-handed orthographic projection matrix with `[-1,1]` depth
        --
        --range.  This is the same as the OpenGL `glOrtho` function in OpenGL.
        --
        --See
        --
        --<https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml>

        orthographic_rh_gl : function((number),(number),(number),(number),(number),(number)):(LuaMat4)

        --Creates a left-handed orthographic projection matrix with `[0,1]` depth range.

        orthographic_lh : function((number),(number),(number),(number),(number),(number)):(LuaMat4)

        --Creates a right-handed orthographic projection matrix with `[0,1]` depth range.

        orthographic_rh : function((number),(number),(number),(number),(number),(number)):(LuaMat4)

        __tostring : function(LuaMat4):(string)

        __unm : function(LuaMat4):(LuaMat4)

        __index : function(LuaMat4,integer):(LuaVec4)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaMat4Class

        userdata
    
    --Collection of static methods for [`LuaMat4`].

        --Creates a 4x4 matrix from four column vectors.

        from_cols : function((LuaVec4),(LuaVec4),(LuaVec4),(LuaVec4)):(LuaMat4)

        --Creates a 4x4 matrix with its diagonal set to `diagonal` and all other entries set to 0.

        from_diagonal : function(LuaVec4):(LuaMat4)

        --Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
        --
        --`translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rotation` is not normalized when `glam_assert` is enabled.

        from_scale_rotation_translation : function((LuaVec3),(LuaQuat),(LuaVec3)):(LuaMat4)

        --Creates an affine transformation matrix from the given 3D `translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rotation` is not normalized when `glam_assert` is enabled.

        from_rotation_translation : function((LuaQuat),(LuaVec3)):(LuaMat4)

        --Creates an affine transformation matrix from the given `rotation` quaternion.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rotation` is not normalized when `glam_assert` is enabled.

        from_quat : function(LuaQuat):(LuaMat4)

        --Creates an affine transformation matrix from the given 3x3 linear transformation
        --
        --matrix.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_mat3 : function(LuaMat3):(LuaMat4)

        --Creates an affine transformation matrix from the given 3x3 linear transformation
        --
        --matrix.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_mat3a : function(LuaMat3A):(LuaMat4)

        --Creates an affine transformation matrix from the given 3D `translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_translation : function(LuaVec3):(LuaMat4)

        --Creates an affine transformation matrix containing a 3D rotation around a normalized
        --
        --rotation `axis` of `angle` (in radians).
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `axis` is not normalized when `glam_assert` is enabled.

        from_axis_angle : function((LuaVec3),(number)):(LuaMat4)

        --Creates a affine transformation matrix containing a rotation from the given euler
        --
        --rotation sequence and angles (in radians).
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_euler : function((LuaEulerRot),(number),(number),(number)):(LuaMat4)

        --Creates an affine transformation matrix containing a 3D rotation around the x axis of
        --
        --`angle` (in radians).
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_rotation_x : function(number):(LuaMat4)

        --Creates an affine transformation matrix containing a 3D rotation around the y axis of
        --
        --`angle` (in radians).
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_rotation_y : function(number):(LuaMat4)

        --Creates an affine transformation matrix containing a 3D rotation around the z axis of
        --
        --`angle` (in radians).
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_rotation_z : function(number):(LuaMat4)

        --Creates an affine transformation matrix containing the given 3D non-uniform `scale`.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if all elements of `scale` are zero when `glam_assert` is enabled.

        from_scale : function(LuaVec3):(LuaMat4)

        --Creates a left-handed view matrix using a camera position, an up direction, and a facing
        --
        --direction.
        --
        --
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.

        look_to_lh : function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaMat4)

        --Creates a right-handed view matrix using a camera position, an up direction, and a facing
        --
        --direction.
        --
        --
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.

        look_to_rh : function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaMat4)

        --Creates a left-handed view matrix using a camera position, an up direction, and a focal
        --
        --point.
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `up` is not normalized when `glam_assert` is enabled.

        look_at_lh : function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaMat4)

        --Creates a right-handed view matrix using a camera position, an up direction, and a focal
        --
        --point.
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `up` is not normalized when `glam_assert` is enabled.

        look_at_rh : function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaMat4)

        --Creates a right-handed perspective projection matrix with [-1,1] depth range.
        --
        --This is the same as the OpenGL `gluPerspective` function.
        --
        --See <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml>

        perspective_rh_gl : function((number),(number),(number),(number)):(LuaMat4)

        --Creates a left-handed perspective projection matrix with `[0,1]` depth range.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
        --
        --enabled.

        perspective_lh : function((number),(number),(number),(number)):(LuaMat4)

        --Creates a right-handed perspective projection matrix with `[0,1]` depth range.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
        --
        --enabled.

        perspective_rh : function((number),(number),(number),(number)):(LuaMat4)

        --Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.

        perspective_infinite_lh : function((number),(number),(number)):(LuaMat4)

        --Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.

        perspective_infinite_reverse_lh : function((number),(number),(number)):(LuaMat4)

        --Creates an infinite right-handed perspective projection matrix with
        --
        --`[0,1]` depth range.

        perspective_infinite_rh : function((number),(number),(number)):(LuaMat4)

        --Creates an infinite reverse right-handed perspective projection matrix
        --
        --with `[0,1]` depth range.

        perspective_infinite_reverse_rh : function((number),(number),(number)):(LuaMat4)

        --Creates a right-handed orthographic projection matrix with `[-1,1]` depth
        --
        --range.  This is the same as the OpenGL `glOrtho` function in OpenGL.
        --
        --See
        --
        --<https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml>

        orthographic_rh_gl : function((number),(number),(number),(number),(number),(number)):(LuaMat4)

        --Creates a left-handed orthographic projection matrix with `[0,1]` depth range.

        orthographic_lh : function((number),(number),(number),(number),(number),(number)):(LuaMat4)

        --Creates a right-handed orthographic projection matrix with `[0,1]` depth range.

        orthographic_rh : function((number),(number),(number),(number),(number),(number)):(LuaMat4)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaDMat2

        userdata
    
    --

        x_axis : LuaDVec2

        y_axis : LuaDVec2

        --Returns the matrix column for the given `index`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Panics if `index` is greater than 1.

        col : function(LuaDMat2,integer):(LuaDVec2)

        --Returns the matrix row for the given `index`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Panics if `index` is greater than 1.

        row : function(LuaDMat2,integer):(LuaDVec2)

        --Returns `true` if, and only if, all elements are finite.
        --
        --If any element is either `NaN`, positive or negative infinity, this will return `false`.

        is_finite : function(LuaDMat2):(boolean)

        --Returns `true` if any elements are `NaN`.

        is_nan : function(LuaDMat2):(boolean)

        --Returns the transpose of `self`.

        transpose : function(LuaDMat2):(LuaDMat2)

        --Returns the determinant of `self`.

        determinant : function(LuaDMat2):(number)

        --Returns the inverse of `self`.
        --
        --
        --
        --If the matrix is not invertible the returned matrix will be invalid.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if the determinant of `self` is zero when `glam_assert` is enabled.

        inverse : function(LuaDMat2):(LuaDMat2)

        --Transforms a 2D vector.

        mul_vec2 : function(LuaDMat2,LuaDVec2):(LuaDVec2)

        --Multiplies two 2x2 matrices.

        mul_mat2 : function(LuaDMat2,LuaDMat2):(LuaDMat2)

        --Adds two 2x2 matrices.

        add_mat2 : function(LuaDMat2,LuaDMat2):(LuaDMat2)

        --Subtracts two 2x2 matrices.

        sub_mat2 : function(LuaDMat2,LuaDMat2):(LuaDMat2)

        --Multiplies a 2x2 matrix by a scalar.

        mul_scalar : function(LuaDMat2,number):(LuaDMat2)

        --Returns true if the absolute difference of all elements between `self` and `rhs`
        --
        --is less than or equal to `max_abs_diff`.
        --
        --
        --
        --This can be used to compare if two matrices contain similar elements. It works best
        --
        --when comparing with a known value. The `max_abs_diff` that should be used used
        --
        --depends on the values being compared against.
        --
        --
        --
        --For more see
        --
        --[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

        abs_diff_eq : function(LuaDMat2,(LuaDMat2),(number)):(boolean)

        as_mat2 : function(LuaDMat2):(LuaMat2)

        --Creates a 2x2 matrix from two column vectors.

        from_cols : function((LuaDVec2),(LuaDVec2)):(LuaDMat2)

        --Creates a 2x2 matrix with its diagonal set to `diagonal` and all other entries set to 0.

        from_diagonal : function(LuaDVec2):(LuaDMat2)

        --Creates a 2x2 matrix containing the combining non-uniform `scale` and rotation of
        --
        --`angle` (in radians).

        from_scale_angle : function((LuaDVec2),(number)):(LuaDMat2)

        --Creates a 2x2 matrix containing a rotation of `angle` (in radians).

        from_angle : function(number):(LuaDMat2)

        --Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.

        from_mat3 : function(LuaDMat3):(LuaDMat2)

        __tostring : function(LuaDMat2):(string)

        __unm : function(LuaDMat2):(LuaDMat2)

        __index : function(LuaDMat2,integer):(LuaDVec2)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaDMat2Class

        userdata
    
    --Collection of static methods for [`LuaDMat2`].

        --Creates a 2x2 matrix from two column vectors.

        from_cols : function((LuaDVec2),(LuaDVec2)):(LuaDMat2)

        --Creates a 2x2 matrix with its diagonal set to `diagonal` and all other entries set to 0.

        from_diagonal : function(LuaDVec2):(LuaDMat2)

        --Creates a 2x2 matrix containing the combining non-uniform `scale` and rotation of
        --
        --`angle` (in radians).

        from_scale_angle : function((LuaDVec2),(number)):(LuaDMat2)

        --Creates a 2x2 matrix containing a rotation of `angle` (in radians).

        from_angle : function(number):(LuaDMat2)

        --Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.

        from_mat3 : function(LuaDMat3):(LuaDMat2)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaDMat3

        userdata
    
    --

        x_axis : LuaDVec3

        y_axis : LuaDVec3

        z_axis : LuaDVec3

        --Returns the matrix column for the given `index`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Panics if `index` is greater than 2.

        col : function(LuaDMat3,integer):(LuaDVec3)

        --Returns the matrix row for the given `index`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Panics if `index` is greater than 2.

        row : function(LuaDMat3,integer):(LuaDVec3)

        --Returns `true` if, and only if, all elements are finite.
        --
        --If any element is either `NaN`, positive or negative infinity, this will return `false`.

        is_finite : function(LuaDMat3):(boolean)

        --Returns `true` if any elements are `NaN`.

        is_nan : function(LuaDMat3):(boolean)

        --Returns the transpose of `self`.

        transpose : function(LuaDMat3):(LuaDMat3)

        --Returns the determinant of `self`.

        determinant : function(LuaDMat3):(number)

        --Returns the inverse of `self`.
        --
        --
        --
        --If the matrix is not invertible the returned matrix will be invalid.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if the determinant of `self` is zero when `glam_assert` is enabled.

        inverse : function(LuaDMat3):(LuaDMat3)

        --Transforms the given 2D vector as a point.
        --
        --
        --
        --This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `1`.
        --
        --
        --
        --This method assumes that `self` contains a valid affine transform.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.

        transform_point2 : function(LuaDMat3,LuaDVec2):(LuaDVec2)

        --Rotates the given 2D vector.
        --
        --
        --
        --This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `0`.
        --
        --
        --
        --This method assumes that `self` contains a valid affine transform.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.

        transform_vector2 : function(LuaDMat3,LuaDVec2):(LuaDVec2)

        --Transforms a 3D vector.

        mul_vec3 : function(LuaDMat3,LuaDVec3):(LuaDVec3)

        --Multiplies two 3x3 matrices.

        mul_mat3 : function(LuaDMat3,LuaDMat3):(LuaDMat3)

        --Adds two 3x3 matrices.

        add_mat3 : function(LuaDMat3,LuaDMat3):(LuaDMat3)

        --Subtracts two 3x3 matrices.

        sub_mat3 : function(LuaDMat3,LuaDMat3):(LuaDMat3)

        --Multiplies a 3x3 matrix by a scalar.

        mul_scalar : function(LuaDMat3,number):(LuaDMat3)

        --Returns true if the absolute difference of all elements between `self` and `rhs`
        --
        --is less than or equal to `max_abs_diff`.
        --
        --
        --
        --This can be used to compare if two matrices contain similar elements. It works best
        --
        --when comparing with a known value. The `max_abs_diff` that should be used used
        --
        --depends on the values being compared against.
        --
        --
        --
        --For more see
        --
        --[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

        abs_diff_eq : function(LuaDMat3,(LuaDMat3),(number)):(boolean)

        as_mat3 : function(LuaDMat3):(LuaMat3)

        --Creates a 3x3 matrix from three column vectors.

        from_cols : function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDMat3)

        --Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.

        from_diagonal : function(LuaDVec3):(LuaDMat3)

        --Creates a 3x3 matrix from a 4x4 matrix, discarding the 4th row and column.

        from_mat4 : function(LuaDMat4):(LuaDMat3)

        --Creates a 3D rotation matrix from the given quaternion.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rotation` is not normalized when `glam_assert` is enabled.

        from_quat : function(LuaDQuat):(LuaDMat3)

        --Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
        --
        --radians).
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `axis` is not normalized when `glam_assert` is enabled.

        from_axis_angle : function((LuaDVec3),(number)):(LuaDMat3)

        --Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
        --
        --radians).

        from_euler : function((LuaEulerRot),(number),(number),(number)):(LuaDMat3)

        --Creates a 3D rotation matrix from `angle` (in radians) around the x axis.

        from_rotation_x : function(number):(LuaDMat3)

        --Creates a 3D rotation matrix from `angle` (in radians) around the y axis.

        from_rotation_y : function(number):(LuaDMat3)

        --Creates a 3D rotation matrix from `angle` (in radians) around the z axis.

        from_rotation_z : function(number):(LuaDMat3)

        --Creates an affine transformation matrix from the given 2D `translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_translation : function(LuaDVec2):(LuaDMat3)

        --Creates an affine transformation matrix from the given 2D rotation `angle` (in
        --
        --radians).
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_angle : function(number):(LuaDMat3)

        --Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
        --
        --radians) and `translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_scale_angle_translation : function((LuaDVec2),(number),(LuaDVec2)):(LuaDMat3)

        --Creates an affine transformation matrix from the given non-uniform 2D `scale`.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if all elements of `scale` are zero when `glam_assert` is enabled.

        from_scale : function(LuaDVec2):(LuaDMat3)

        --Creates an affine transformation matrix from the given 2x2 matrix.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_mat2 : function(LuaDMat2):(LuaDMat3)

        __tostring : function(LuaDMat3):(string)

        __unm : function(LuaDMat3):(LuaDMat3)

        __index : function(LuaDMat3,integer):(LuaDVec3)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaDMat3Class

        userdata
    
    --Collection of static methods for [`LuaDMat3`].

        --Creates a 3x3 matrix from three column vectors.

        from_cols : function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDMat3)

        --Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.

        from_diagonal : function(LuaDVec3):(LuaDMat3)

        --Creates a 3x3 matrix from a 4x4 matrix, discarding the 4th row and column.

        from_mat4 : function(LuaDMat4):(LuaDMat3)

        --Creates a 3D rotation matrix from the given quaternion.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rotation` is not normalized when `glam_assert` is enabled.

        from_quat : function(LuaDQuat):(LuaDMat3)

        --Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
        --
        --radians).
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `axis` is not normalized when `glam_assert` is enabled.

        from_axis_angle : function((LuaDVec3),(number)):(LuaDMat3)

        --Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
        --
        --radians).

        from_euler : function((LuaEulerRot),(number),(number),(number)):(LuaDMat3)

        --Creates a 3D rotation matrix from `angle` (in radians) around the x axis.

        from_rotation_x : function(number):(LuaDMat3)

        --Creates a 3D rotation matrix from `angle` (in radians) around the y axis.

        from_rotation_y : function(number):(LuaDMat3)

        --Creates a 3D rotation matrix from `angle` (in radians) around the z axis.

        from_rotation_z : function(number):(LuaDMat3)

        --Creates an affine transformation matrix from the given 2D `translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_translation : function(LuaDVec2):(LuaDMat3)

        --Creates an affine transformation matrix from the given 2D rotation `angle` (in
        --
        --radians).
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_angle : function(number):(LuaDMat3)

        --Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
        --
        --radians) and `translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_scale_angle_translation : function((LuaDVec2),(number),(LuaDVec2)):(LuaDMat3)

        --Creates an affine transformation matrix from the given non-uniform 2D `scale`.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if all elements of `scale` are zero when `glam_assert` is enabled.

        from_scale : function(LuaDVec2):(LuaDMat3)

        --Creates an affine transformation matrix from the given 2x2 matrix.
        --
        --
        --
        --The resulting matrix can be used to transform 2D points and vectors. See
        --
        --[`Self::transform_point2()`] and [`Self::transform_vector2()`].

        from_mat2 : function(LuaDMat2):(LuaDMat3)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaDMat4

        userdata
    
    --

        x_axis : LuaDVec4

        y_axis : LuaDVec4

        z_axis : LuaDVec4

        w_axis : LuaDVec4

        --Returns the matrix column for the given `index`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Panics if `index` is greater than 3.

        col : function(LuaDMat4,integer):(LuaDVec4)

        --Returns the matrix row for the given `index`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Panics if `index` is greater than 3.

        row : function(LuaDMat4,integer):(LuaDVec4)

        --Returns `true` if, and only if, all elements are finite.
        --
        --If any element is either `NaN`, positive or negative infinity, this will return `false`.

        is_finite : function(LuaDMat4):(boolean)

        --Returns `true` if any elements are `NaN`.

        is_nan : function(LuaDMat4):(boolean)

        --Returns the transpose of `self`.

        transpose : function(LuaDMat4):(LuaDMat4)

        --Returns the determinant of `self`.

        determinant : function(LuaDMat4):(number)

        --Returns the inverse of `self`.
        --
        --
        --
        --If the matrix is not invertible the returned matrix will be invalid.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if the determinant of `self` is zero when `glam_assert` is enabled.

        inverse : function(LuaDMat4):(LuaDMat4)

        --Transforms the given 3D vector as a point, applying perspective correction.
        --
        --
        --
        --This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is `1.0`.
        --
        --The perspective divide is performed meaning the resulting 3D vector is divided by `w`.
        --
        --
        --
        --This method assumes that `self` contains a projective transform.

        project_point3 : function(LuaDMat4,LuaDVec3):(LuaDVec3)

        --Transforms the given 3D vector as a point.
        --
        --
        --
        --This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
        --
        --`1.0`.
        --
        --
        --
        --This method assumes that `self` contains a valid affine transform. It does not perform
        --
        --a persective divide, if `self` contains a perspective transform, or if you are unsure,
        --
        --the [`Self::project_point3()`] method should be used instead.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.

        transform_point3 : function(LuaDMat4,LuaDVec3):(LuaDVec3)

        --Transforms the give 3D vector as a direction.
        --
        --
        --
        --This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
        --
        --`0.0`.
        --
        --
        --
        --This method assumes that `self` contains a valid affine transform.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.

        transform_vector3 : function(LuaDMat4,LuaDVec3):(LuaDVec3)

        --Transforms a 4D vector.

        mul_vec4 : function(LuaDMat4,LuaDVec4):(LuaDVec4)

        --Multiplies two 4x4 matrices.

        mul_mat4 : function(LuaDMat4,LuaDMat4):(LuaDMat4)

        --Adds two 4x4 matrices.

        add_mat4 : function(LuaDMat4,LuaDMat4):(LuaDMat4)

        --Subtracts two 4x4 matrices.

        sub_mat4 : function(LuaDMat4,LuaDMat4):(LuaDMat4)

        --Multiplies a 4x4 matrix by a scalar.

        mul_scalar : function(LuaDMat4,number):(LuaDMat4)

        --Returns true if the absolute difference of all elements between `self` and `rhs`
        --
        --is less than or equal to `max_abs_diff`.
        --
        --
        --
        --This can be used to compare if two matrices contain similar elements. It works best
        --
        --when comparing with a known value. The `max_abs_diff` that should be used used
        --
        --depends on the values being compared against.
        --
        --
        --
        --For more see
        --
        --[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

        abs_diff_eq : function(LuaDMat4,(LuaDMat4),(number)):(boolean)

        as_mat4 : function(LuaDMat4):(LuaMat4)

        --Creates a 4x4 matrix from four column vectors.

        from_cols : function((LuaDVec4),(LuaDVec4),(LuaDVec4),(LuaDVec4)):(LuaDMat4)

        --Creates a 4x4 matrix with its diagonal set to `diagonal` and all other entries set to 0.

        from_diagonal : function(LuaDVec4):(LuaDMat4)

        --Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
        --
        --`translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rotation` is not normalized when `glam_assert` is enabled.

        from_scale_rotation_translation : function((LuaDVec3),(LuaDQuat),(LuaDVec3)):(LuaDMat4)

        --Creates an affine transformation matrix from the given 3D `translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rotation` is not normalized when `glam_assert` is enabled.

        from_rotation_translation : function((LuaDQuat),(LuaDVec3)):(LuaDMat4)

        --Creates an affine transformation matrix from the given `rotation` quaternion.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rotation` is not normalized when `glam_assert` is enabled.

        from_quat : function(LuaDQuat):(LuaDMat4)

        --Creates an affine transformation matrix from the given 3x3 linear transformation
        --
        --matrix.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_mat3 : function(LuaDMat3):(LuaDMat4)

        --Creates an affine transformation matrix from the given 3D `translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_translation : function(LuaDVec3):(LuaDMat4)

        --Creates an affine transformation matrix containing a 3D rotation around a normalized
        --
        --rotation `axis` of `angle` (in radians).
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `axis` is not normalized when `glam_assert` is enabled.

        from_axis_angle : function((LuaDVec3),(number)):(LuaDMat4)

        --Creates a affine transformation matrix containing a rotation from the given euler
        --
        --rotation sequence and angles (in radians).
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_euler : function((LuaEulerRot),(number),(number),(number)):(LuaDMat4)

        --Creates an affine transformation matrix containing a 3D rotation around the x axis of
        --
        --`angle` (in radians).
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_rotation_x : function(number):(LuaDMat4)

        --Creates an affine transformation matrix containing a 3D rotation around the y axis of
        --
        --`angle` (in radians).
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_rotation_y : function(number):(LuaDMat4)

        --Creates an affine transformation matrix containing a 3D rotation around the z axis of
        --
        --`angle` (in radians).
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_rotation_z : function(number):(LuaDMat4)

        --Creates an affine transformation matrix containing the given 3D non-uniform `scale`.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if all elements of `scale` are zero when `glam_assert` is enabled.

        from_scale : function(LuaDVec3):(LuaDMat4)

        --Creates a left-handed view matrix using a camera position, an up direction, and a facing
        --
        --direction.
        --
        --
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.

        look_to_lh : function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDMat4)

        --Creates a right-handed view matrix using a camera position, an up direction, and a facing
        --
        --direction.
        --
        --
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.

        look_to_rh : function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDMat4)

        --Creates a left-handed view matrix using a camera position, an up direction, and a focal
        --
        --point.
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `up` is not normalized when `glam_assert` is enabled.

        look_at_lh : function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDMat4)

        --Creates a right-handed view matrix using a camera position, an up direction, and a focal
        --
        --point.
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `up` is not normalized when `glam_assert` is enabled.

        look_at_rh : function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDMat4)

        --Creates a right-handed perspective projection matrix with [-1,1] depth range.
        --
        --This is the same as the OpenGL `gluPerspective` function.
        --
        --See <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml>

        perspective_rh_gl : function((number),(number),(number),(number)):(LuaDMat4)

        --Creates a left-handed perspective projection matrix with `[0,1]` depth range.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
        --
        --enabled.

        perspective_lh : function((number),(number),(number),(number)):(LuaDMat4)

        --Creates a right-handed perspective projection matrix with `[0,1]` depth range.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
        --
        --enabled.

        perspective_rh : function((number),(number),(number),(number)):(LuaDMat4)

        --Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.

        perspective_infinite_lh : function((number),(number),(number)):(LuaDMat4)

        --Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.

        perspective_infinite_reverse_lh : function((number),(number),(number)):(LuaDMat4)

        --Creates an infinite right-handed perspective projection matrix with
        --
        --`[0,1]` depth range.

        perspective_infinite_rh : function((number),(number),(number)):(LuaDMat4)

        --Creates an infinite reverse right-handed perspective projection matrix
        --
        --with `[0,1]` depth range.

        perspective_infinite_reverse_rh : function((number),(number),(number)):(LuaDMat4)

        --Creates a right-handed orthographic projection matrix with `[-1,1]` depth
        --
        --range.  This is the same as the OpenGL `glOrtho` function in OpenGL.
        --
        --See
        --
        --<https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml>

        orthographic_rh_gl : function((number),(number),(number),(number),(number),(number)):(LuaDMat4)

        --Creates a left-handed orthographic projection matrix with `[0,1]` depth range.

        orthographic_lh : function((number),(number),(number),(number),(number),(number)):(LuaDMat4)

        --Creates a right-handed orthographic projection matrix with `[0,1]` depth range.

        orthographic_rh : function((number),(number),(number),(number),(number),(number)):(LuaDMat4)

        __tostring : function(LuaDMat4):(string)

        __unm : function(LuaDMat4):(LuaDMat4)

        __index : function(LuaDMat4,integer):(LuaDVec4)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaDMat4Class

        userdata
    
    --Collection of static methods for [`LuaDMat4`].

        --Creates a 4x4 matrix from four column vectors.

        from_cols : function((LuaDVec4),(LuaDVec4),(LuaDVec4),(LuaDVec4)):(LuaDMat4)

        --Creates a 4x4 matrix with its diagonal set to `diagonal` and all other entries set to 0.

        from_diagonal : function(LuaDVec4):(LuaDMat4)

        --Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
        --
        --`translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rotation` is not normalized when `glam_assert` is enabled.

        from_scale_rotation_translation : function((LuaDVec3),(LuaDQuat),(LuaDVec3)):(LuaDMat4)

        --Creates an affine transformation matrix from the given 3D `translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rotation` is not normalized when `glam_assert` is enabled.

        from_rotation_translation : function((LuaDQuat),(LuaDVec3)):(LuaDMat4)

        --Creates an affine transformation matrix from the given `rotation` quaternion.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `rotation` is not normalized when `glam_assert` is enabled.

        from_quat : function(LuaDQuat):(LuaDMat4)

        --Creates an affine transformation matrix from the given 3x3 linear transformation
        --
        --matrix.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_mat3 : function(LuaDMat3):(LuaDMat4)

        --Creates an affine transformation matrix from the given 3D `translation`.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_translation : function(LuaDVec3):(LuaDMat4)

        --Creates an affine transformation matrix containing a 3D rotation around a normalized
        --
        --rotation `axis` of `angle` (in radians).
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `axis` is not normalized when `glam_assert` is enabled.

        from_axis_angle : function((LuaDVec3),(number)):(LuaDMat4)

        --Creates a affine transformation matrix containing a rotation from the given euler
        --
        --rotation sequence and angles (in radians).
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_euler : function((LuaEulerRot),(number),(number),(number)):(LuaDMat4)

        --Creates an affine transformation matrix containing a 3D rotation around the x axis of
        --
        --`angle` (in radians).
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_rotation_x : function(number):(LuaDMat4)

        --Creates an affine transformation matrix containing a 3D rotation around the y axis of
        --
        --`angle` (in radians).
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_rotation_y : function(number):(LuaDMat4)

        --Creates an affine transformation matrix containing a 3D rotation around the z axis of
        --
        --`angle` (in radians).
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].

        from_rotation_z : function(number):(LuaDMat4)

        --Creates an affine transformation matrix containing the given 3D non-uniform `scale`.
        --
        --
        --
        --The resulting matrix can be used to transform 3D points and vectors. See
        --
        --[`Self::transform_point3()`] and [`Self::transform_vector3()`].
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if all elements of `scale` are zero when `glam_assert` is enabled.

        from_scale : function(LuaDVec3):(LuaDMat4)

        --Creates a left-handed view matrix using a camera position, an up direction, and a facing
        --
        --direction.
        --
        --
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.

        look_to_lh : function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDMat4)

        --Creates a right-handed view matrix using a camera position, an up direction, and a facing
        --
        --direction.
        --
        --
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.

        look_to_rh : function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDMat4)

        --Creates a left-handed view matrix using a camera position, an up direction, and a focal
        --
        --point.
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `up` is not normalized when `glam_assert` is enabled.

        look_at_lh : function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDMat4)

        --Creates a right-handed view matrix using a camera position, an up direction, and a focal
        --
        --point.
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `up` is not normalized when `glam_assert` is enabled.

        look_at_rh : function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDMat4)

        --Creates a right-handed perspective projection matrix with [-1,1] depth range.
        --
        --This is the same as the OpenGL `gluPerspective` function.
        --
        --See <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml>

        perspective_rh_gl : function((number),(number),(number),(number)):(LuaDMat4)

        --Creates a left-handed perspective projection matrix with `[0,1]` depth range.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
        --
        --enabled.

        perspective_lh : function((number),(number),(number),(number)):(LuaDMat4)

        --Creates a right-handed perspective projection matrix with `[0,1]` depth range.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
        --
        --enabled.

        perspective_rh : function((number),(number),(number),(number)):(LuaDMat4)

        --Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.

        perspective_infinite_lh : function((number),(number),(number)):(LuaDMat4)

        --Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.

        perspective_infinite_reverse_lh : function((number),(number),(number)):(LuaDMat4)

        --Creates an infinite right-handed perspective projection matrix with
        --
        --`[0,1]` depth range.

        perspective_infinite_rh : function((number),(number),(number)):(LuaDMat4)

        --Creates an infinite reverse right-handed perspective projection matrix
        --
        --with `[0,1]` depth range.

        perspective_infinite_reverse_rh : function((number),(number),(number)):(LuaDMat4)

        --Creates a right-handed orthographic projection matrix with `[-1,1]` depth
        --
        --range.  This is the same as the OpenGL `glOrtho` function in OpenGL.
        --
        --See
        --
        --<https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml>

        orthographic_rh_gl : function((number),(number),(number),(number),(number),(number)):(LuaDMat4)

        --Creates a left-handed orthographic projection matrix with `[0,1]` depth range.

        orthographic_lh : function((number),(number),(number),(number),(number),(number)):(LuaDMat4)

        --Creates a right-handed orthographic projection matrix with `[0,1]` depth range.

        orthographic_rh : function((number),(number),(number),(number),(number),(number)):(LuaDMat4)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaAffine2

        userdata
    
    --

        matrix2 : LuaMat2

        translation : LuaVec2

        --Transforms the given 2D point, applying shear, scale, rotation and translation.

        transform_point2 : function(LuaAffine2,LuaVec2):(LuaVec2)

        --Transforms the given 2D vector, applying shear, scale and rotation (but NOT
        --
        --translation).
        --
        --
        --
        --To also apply translation, use [`Self::transform_point2()`] instead.

        transform_vector2 : function(LuaAffine2,LuaVec2):(LuaVec2)

        --Returns `true` if, and only if, all elements are finite.
        --
        --
        --
        --If any element is either `NaN`, positive or negative infinity, this will return
        --
        --`false`.

        is_finite : function(LuaAffine2):(boolean)

        --Returns `true` if any elements are `NaN`.

        is_nan : function(LuaAffine2):(boolean)

        --Returns true if the absolute difference of all elements between `self` and `rhs`
        --
        --is less than or equal to `max_abs_diff`.
        --
        --
        --
        --This can be used to compare if two 3x4 matrices contain similar elements. It works
        --
        --best when comparing with a known value. The `max_abs_diff` that should be used used
        --
        --depends on the values being compared against.
        --
        --
        --
        --For more see
        --
        --[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

        abs_diff_eq : function(LuaAffine2,(LuaAffine2),(number)):(boolean)

        --Return the inverse of this transform.
        --
        --
        --
        --Note that if the transform is not invertible the result will be invalid.

        inverse : function(LuaAffine2):(LuaAffine2)

        --Creates an affine transform from three column vectors.

        from_cols : function((LuaVec2),(LuaVec2),(LuaVec2)):(LuaAffine2)

        --Creates an affine transform that changes scale.
        --
        --Note that if any scale is zero the transform will be non-invertible.

        from_scale : function(LuaVec2):(LuaAffine2)

        --Creates an affine transform from the given rotation `angle`.

        from_angle : function(number):(LuaAffine2)

        --Creates an affine transformation from the given 2D `translation`.

        from_translation : function(LuaVec2):(LuaAffine2)

        --Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation)

        from_mat2 : function(LuaMat2):(LuaAffine2)

        --Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation) and a
        --
        --translation vector.
        --
        --
        --
        --Equivalent to
        --
        --`Affine2::from_translation(translation) * Affine2::from_mat2(mat2)`

        from_mat2_translation : function((LuaMat2),(LuaVec2)):(LuaAffine2)

        --Creates an affine transform from the given 2D `scale`, rotation `angle` (in radians) and
        --
        --`translation`.
        --
        --
        --
        --Equivalent to `Affine2::from_translation(translation) *
        --
        --Affine2::from_angle(angle) * Affine2::from_scale(scale)`

        from_scale_angle_translation : function((LuaVec2),(number),(LuaVec2)):(LuaAffine2)

        --Creates an affine transform from the given 2D rotation `angle` (in radians) and
        --
        --`translation`.
        --
        --
        --
        --Equivalent to `Affine2::from_translation(translation) * Affine2::from_angle(angle)`

        from_angle_translation : function((number),(LuaVec2)):(LuaAffine2)

        --The given `Mat3` must be an affine transform,

        from_mat3 : function(LuaMat3):(LuaAffine2)

        --The given [`Mat3A`] must be an affine transform,

        from_mat3a : function(LuaMat3A):(LuaAffine2)

        __tostring : function(LuaAffine2):(string)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaAffine2Class

        userdata
    
    --Collection of static methods for [`LuaAffine2`].

        --Creates an affine transform from three column vectors.

        from_cols : function((LuaVec2),(LuaVec2),(LuaVec2)):(LuaAffine2)

        --Creates an affine transform that changes scale.
        --
        --Note that if any scale is zero the transform will be non-invertible.

        from_scale : function(LuaVec2):(LuaAffine2)

        --Creates an affine transform from the given rotation `angle`.

        from_angle : function(number):(LuaAffine2)

        --Creates an affine transformation from the given 2D `translation`.

        from_translation : function(LuaVec2):(LuaAffine2)

        --Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation)

        from_mat2 : function(LuaMat2):(LuaAffine2)

        --Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation) and a
        --
        --translation vector.
        --
        --
        --
        --Equivalent to
        --
        --`Affine2::from_translation(translation) * Affine2::from_mat2(mat2)`

        from_mat2_translation : function((LuaMat2),(LuaVec2)):(LuaAffine2)

        --Creates an affine transform from the given 2D `scale`, rotation `angle` (in radians) and
        --
        --`translation`.
        --
        --
        --
        --Equivalent to `Affine2::from_translation(translation) *
        --
        --Affine2::from_angle(angle) * Affine2::from_scale(scale)`

        from_scale_angle_translation : function((LuaVec2),(number),(LuaVec2)):(LuaAffine2)

        --Creates an affine transform from the given 2D rotation `angle` (in radians) and
        --
        --`translation`.
        --
        --
        --
        --Equivalent to `Affine2::from_translation(translation) * Affine2::from_angle(angle)`

        from_angle_translation : function((number),(LuaVec2)):(LuaAffine2)

        --The given `Mat3` must be an affine transform,

        from_mat3 : function(LuaMat3):(LuaAffine2)

        --The given [`Mat3A`] must be an affine transform,

        from_mat3a : function(LuaMat3A):(LuaAffine2)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaAffine3A

        userdata
    
    --

        matrix3 : LuaMat3A

        translation : LuaVec3A

        --Transforms the given 3D points, applying shear, scale, rotation and translation.

        transform_point3 : function(LuaAffine3A,LuaVec3):(LuaVec3)

        --Transforms the given 3D vector, applying shear, scale and rotation (but NOT
        --
        --translation).
        --
        --
        --
        --To also apply translation, use [`Self::transform_point3()`] instead.

        transform_vector3 : function(LuaAffine3A,LuaVec3):(LuaVec3)

        --Transforms the given [`Vec3A`], applying shear, scale, rotation and translation.

        transform_point3a : function(LuaAffine3A,LuaVec3A):(LuaVec3A)

        --Transforms the given [`Vec3A`], applying shear, scale and rotation (but NOT
        --
        --translation).
        --
        --
        --
        --To also apply translation, use [`Self::transform_point3a()`] instead.

        transform_vector3a : function(LuaAffine3A,LuaVec3A):(LuaVec3A)

        --Returns `true` if, and only if, all elements are finite.
        --
        --
        --
        --If any element is either `NaN`, positive or negative infinity, this will return
        --
        --`false`.

        is_finite : function(LuaAffine3A):(boolean)

        --Returns `true` if any elements are `NaN`.

        is_nan : function(LuaAffine3A):(boolean)

        --Returns true if the absolute difference of all elements between `self` and `rhs`
        --
        --is less than or equal to `max_abs_diff`.
        --
        --
        --
        --This can be used to compare if two 3x4 matrices contain similar elements. It works
        --
        --best when comparing with a known value. The `max_abs_diff` that should be used used
        --
        --depends on the values being compared against.
        --
        --
        --
        --For more see
        --
        --[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

        abs_diff_eq : function(LuaAffine3A,(LuaAffine3A),(number)):(boolean)

        --Return the inverse of this transform.
        --
        --
        --
        --Note that if the transform is not invertible the result will be invalid.

        inverse : function(LuaAffine3A):(LuaAffine3A)

        --Creates an affine transform from three column vectors.

        from_cols : function((LuaVec3A),(LuaVec3A),(LuaVec3A),(LuaVec3A)):(LuaAffine3A)

        --Creates an affine transform that changes scale.
        --
        --Note that if any scale is zero the transform will be non-invertible.

        from_scale : function(LuaVec3):(LuaAffine3A)

        --Creates an affine transform from the given `rotation` quaternion.

        from_quat : function(LuaQuat):(LuaAffine3A)

        --Creates an affine transform containing a 3D rotation around a normalized
        --
        --rotation `axis` of `angle` (in radians).

        from_axis_angle : function((LuaVec3),(number)):(LuaAffine3A)

        --Creates an affine transform containing a 3D rotation around the x axis of
        --
        --`angle` (in radians).

        from_rotation_x : function(number):(LuaAffine3A)

        --Creates an affine transform containing a 3D rotation around the y axis of
        --
        --`angle` (in radians).

        from_rotation_y : function(number):(LuaAffine3A)

        --Creates an affine transform containing a 3D rotation around the z axis of
        --
        --`angle` (in radians).

        from_rotation_z : function(number):(LuaAffine3A)

        --Creates an affine transformation from the given 3D `translation`.

        from_translation : function(LuaVec3):(LuaAffine3A)

        --Creates an affine transform from a 3x3 matrix (expressing scale, shear and
        --
        --rotation)

        from_mat3 : function(LuaMat3):(LuaAffine3A)

        --Creates an affine transform from a 3x3 matrix (expressing scale, shear and rotation)
        --
        --and a translation vector.
        --
        --
        --
        --Equivalent to `Affine3A::from_translation(translation) * Affine3A::from_mat3(mat3)`

        from_mat3_translation : function((LuaMat3),(LuaVec3)):(LuaAffine3A)

        --Creates an affine transform from the given 3D `scale`, `rotation` and
        --
        --`translation`.
        --
        --
        --
        --Equivalent to `Affine3A::from_translation(translation) *
        --
        --Affine3A::from_quat(rotation) * Affine3A::from_scale(scale)`

        from_scale_rotation_translation : function((LuaVec3),(LuaQuat),(LuaVec3)):(LuaAffine3A)

        --Creates an affine transform from the given 3D `rotation` and `translation`.
        --
        --
        --
        --Equivalent to `Affine3A::from_translation(translation) * Affine3A::from_quat(rotation)`

        from_rotation_translation : function((LuaQuat),(LuaVec3)):(LuaAffine3A)

        --The given `Mat4` must be an affine transform,
        --
        --i.e. contain no perspective transform.

        from_mat4 : function(LuaMat4):(LuaAffine3A)

        --Creates a left-handed view transform using a camera position, an up direction, and a facing
        --
        --direction.
        --
        --
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.

        look_to_lh : function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaAffine3A)

        --Creates a right-handed view transform using a camera position, an up direction, and a facing
        --
        --direction.
        --
        --
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.

        look_to_rh : function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaAffine3A)

        --Creates a left-handed view transform using a camera position, an up direction, and a focal
        --
        --point.
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `up` is not normalized when `glam_assert` is enabled.

        look_at_lh : function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaAffine3A)

        --Creates a right-handed view transform using a camera position, an up direction, and a focal
        --
        --point.
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `up` is not normalized when `glam_assert` is enabled.

        look_at_rh : function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaAffine3A)

        __tostring : function(LuaAffine3A):(string)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaAffine3AClass

        userdata
    
    --Collection of static methods for [`LuaAffine3A`].

        --Creates an affine transform from three column vectors.

        from_cols : function((LuaVec3A),(LuaVec3A),(LuaVec3A),(LuaVec3A)):(LuaAffine3A)

        --Creates an affine transform that changes scale.
        --
        --Note that if any scale is zero the transform will be non-invertible.

        from_scale : function(LuaVec3):(LuaAffine3A)

        --Creates an affine transform from the given `rotation` quaternion.

        from_quat : function(LuaQuat):(LuaAffine3A)

        --Creates an affine transform containing a 3D rotation around a normalized
        --
        --rotation `axis` of `angle` (in radians).

        from_axis_angle : function((LuaVec3),(number)):(LuaAffine3A)

        --Creates an affine transform containing a 3D rotation around the x axis of
        --
        --`angle` (in radians).

        from_rotation_x : function(number):(LuaAffine3A)

        --Creates an affine transform containing a 3D rotation around the y axis of
        --
        --`angle` (in radians).

        from_rotation_y : function(number):(LuaAffine3A)

        --Creates an affine transform containing a 3D rotation around the z axis of
        --
        --`angle` (in radians).

        from_rotation_z : function(number):(LuaAffine3A)

        --Creates an affine transformation from the given 3D `translation`.

        from_translation : function(LuaVec3):(LuaAffine3A)

        --Creates an affine transform from a 3x3 matrix (expressing scale, shear and
        --
        --rotation)

        from_mat3 : function(LuaMat3):(LuaAffine3A)

        --Creates an affine transform from a 3x3 matrix (expressing scale, shear and rotation)
        --
        --and a translation vector.
        --
        --
        --
        --Equivalent to `Affine3A::from_translation(translation) * Affine3A::from_mat3(mat3)`

        from_mat3_translation : function((LuaMat3),(LuaVec3)):(LuaAffine3A)

        --Creates an affine transform from the given 3D `scale`, `rotation` and
        --
        --`translation`.
        --
        --
        --
        --Equivalent to `Affine3A::from_translation(translation) *
        --
        --Affine3A::from_quat(rotation) * Affine3A::from_scale(scale)`

        from_scale_rotation_translation : function((LuaVec3),(LuaQuat),(LuaVec3)):(LuaAffine3A)

        --Creates an affine transform from the given 3D `rotation` and `translation`.
        --
        --
        --
        --Equivalent to `Affine3A::from_translation(translation) * Affine3A::from_quat(rotation)`

        from_rotation_translation : function((LuaQuat),(LuaVec3)):(LuaAffine3A)

        --The given `Mat4` must be an affine transform,
        --
        --i.e. contain no perspective transform.

        from_mat4 : function(LuaMat4):(LuaAffine3A)

        --Creates a left-handed view transform using a camera position, an up direction, and a facing
        --
        --direction.
        --
        --
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.

        look_to_lh : function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaAffine3A)

        --Creates a right-handed view transform using a camera position, an up direction, and a facing
        --
        --direction.
        --
        --
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.

        look_to_rh : function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaAffine3A)

        --Creates a left-handed view transform using a camera position, an up direction, and a focal
        --
        --point.
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `up` is not normalized when `glam_assert` is enabled.

        look_at_lh : function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaAffine3A)

        --Creates a right-handed view transform using a camera position, an up direction, and a focal
        --
        --point.
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `up` is not normalized when `glam_assert` is enabled.

        look_at_rh : function((LuaVec3),(LuaVec3),(LuaVec3)):(LuaAffine3A)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaDAffine2

        userdata
    
    --

        matrix2 : LuaDMat2

        translation : LuaDVec2

        --Transforms the given 2D point, applying shear, scale, rotation and translation.

        transform_point2 : function(LuaDAffine2,LuaDVec2):(LuaDVec2)

        --Transforms the given 2D vector, applying shear, scale and rotation (but NOT
        --
        --translation).
        --
        --
        --
        --To also apply translation, use [`Self::transform_point2()`] instead.

        transform_vector2 : function(LuaDAffine2,LuaDVec2):(LuaDVec2)

        --Returns `true` if, and only if, all elements are finite.
        --
        --
        --
        --If any element is either `NaN`, positive or negative infinity, this will return
        --
        --`false`.

        is_finite : function(LuaDAffine2):(boolean)

        --Returns `true` if any elements are `NaN`.

        is_nan : function(LuaDAffine2):(boolean)

        --Returns true if the absolute difference of all elements between `self` and `rhs`
        --
        --is less than or equal to `max_abs_diff`.
        --
        --
        --
        --This can be used to compare if two 3x4 matrices contain similar elements. It works
        --
        --best when comparing with a known value. The `max_abs_diff` that should be used used
        --
        --depends on the values being compared against.
        --
        --
        --
        --For more see
        --
        --[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

        abs_diff_eq : function(LuaDAffine2,(LuaDAffine2),(number)):(boolean)

        --Return the inverse of this transform.
        --
        --
        --
        --Note that if the transform is not invertible the result will be invalid.

        inverse : function(LuaDAffine2):(LuaDAffine2)

        --Creates an affine transform from three column vectors.

        from_cols : function((LuaDVec2),(LuaDVec2),(LuaDVec2)):(LuaDAffine2)

        --Creates an affine transform that changes scale.
        --
        --Note that if any scale is zero the transform will be non-invertible.

        from_scale : function(LuaDVec2):(LuaDAffine2)

        --Creates an affine transform from the given rotation `angle`.

        from_angle : function(number):(LuaDAffine2)

        --Creates an affine transformation from the given 2D `translation`.

        from_translation : function(LuaDVec2):(LuaDAffine2)

        --Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation)

        from_mat2 : function(LuaDMat2):(LuaDAffine2)

        --Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation) and a
        --
        --translation vector.
        --
        --
        --
        --Equivalent to
        --
        --`DAffine2::from_translation(translation) * DAffine2::from_mat2(mat2)`

        from_mat2_translation : function((LuaDMat2),(LuaDVec2)):(LuaDAffine2)

        --Creates an affine transform from the given 2D `scale`, rotation `angle` (in radians) and
        --
        --`translation`.
        --
        --
        --
        --Equivalent to `DAffine2::from_translation(translation) *
        --
        --DAffine2::from_angle(angle) * DAffine2::from_scale(scale)`

        from_scale_angle_translation : function((LuaDVec2),(number),(LuaDVec2)):(LuaDAffine2)

        --Creates an affine transform from the given 2D rotation `angle` (in radians) and
        --
        --`translation`.
        --
        --
        --
        --Equivalent to `DAffine2::from_translation(translation) * DAffine2::from_angle(angle)`

        from_angle_translation : function((number),(LuaDVec2)):(LuaDAffine2)

        --The given `DMat3` must be an affine transform,

        from_mat3 : function(LuaDMat3):(LuaDAffine2)

        __tostring : function(LuaDAffine2):(string)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaDAffine2Class

        userdata
    
    --Collection of static methods for [`LuaDAffine2`].

        --Creates an affine transform from three column vectors.

        from_cols : function((LuaDVec2),(LuaDVec2),(LuaDVec2)):(LuaDAffine2)

        --Creates an affine transform that changes scale.
        --
        --Note that if any scale is zero the transform will be non-invertible.

        from_scale : function(LuaDVec2):(LuaDAffine2)

        --Creates an affine transform from the given rotation `angle`.

        from_angle : function(number):(LuaDAffine2)

        --Creates an affine transformation from the given 2D `translation`.

        from_translation : function(LuaDVec2):(LuaDAffine2)

        --Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation)

        from_mat2 : function(LuaDMat2):(LuaDAffine2)

        --Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation) and a
        --
        --translation vector.
        --
        --
        --
        --Equivalent to
        --
        --`DAffine2::from_translation(translation) * DAffine2::from_mat2(mat2)`

        from_mat2_translation : function((LuaDMat2),(LuaDVec2)):(LuaDAffine2)

        --Creates an affine transform from the given 2D `scale`, rotation `angle` (in radians) and
        --
        --`translation`.
        --
        --
        --
        --Equivalent to `DAffine2::from_translation(translation) *
        --
        --DAffine2::from_angle(angle) * DAffine2::from_scale(scale)`

        from_scale_angle_translation : function((LuaDVec2),(number),(LuaDVec2)):(LuaDAffine2)

        --Creates an affine transform from the given 2D rotation `angle` (in radians) and
        --
        --`translation`.
        --
        --
        --
        --Equivalent to `DAffine2::from_translation(translation) * DAffine2::from_angle(angle)`

        from_angle_translation : function((number),(LuaDVec2)):(LuaDAffine2)

        --The given `DMat3` must be an affine transform,

        from_mat3 : function(LuaDMat3):(LuaDAffine2)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaDAffine3

        userdata
    
    --

        matrix3 : LuaDMat3

        translation : LuaDVec3

        --Transforms the given 3D points, applying shear, scale, rotation and translation.

        transform_point3 : function(LuaDAffine3,LuaDVec3):(LuaDVec3)

        --Transforms the given 3D vector, applying shear, scale and rotation (but NOT
        --
        --translation).
        --
        --
        --
        --To also apply translation, use [`Self::transform_point3()`] instead.

        transform_vector3 : function(LuaDAffine3,LuaDVec3):(LuaDVec3)

        --Returns `true` if, and only if, all elements are finite.
        --
        --
        --
        --If any element is either `NaN`, positive or negative infinity, this will return
        --
        --`false`.

        is_finite : function(LuaDAffine3):(boolean)

        --Returns `true` if any elements are `NaN`.

        is_nan : function(LuaDAffine3):(boolean)

        --Returns true if the absolute difference of all elements between `self` and `rhs`
        --
        --is less than or equal to `max_abs_diff`.
        --
        --
        --
        --This can be used to compare if two 3x4 matrices contain similar elements. It works
        --
        --best when comparing with a known value. The `max_abs_diff` that should be used used
        --
        --depends on the values being compared against.
        --
        --
        --
        --For more see
        --
        --[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

        abs_diff_eq : function(LuaDAffine3,(LuaDAffine3),(number)):(boolean)

        --Return the inverse of this transform.
        --
        --
        --
        --Note that if the transform is not invertible the result will be invalid.

        inverse : function(LuaDAffine3):(LuaDAffine3)

        --Creates an affine transform from three column vectors.

        from_cols : function((LuaDVec3),(LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDAffine3)

        --Creates an affine transform that changes scale.
        --
        --Note that if any scale is zero the transform will be non-invertible.

        from_scale : function(LuaDVec3):(LuaDAffine3)

        --Creates an affine transform from the given `rotation` quaternion.

        from_quat : function(LuaDQuat):(LuaDAffine3)

        --Creates an affine transform containing a 3D rotation around a normalized
        --
        --rotation `axis` of `angle` (in radians).

        from_axis_angle : function((LuaDVec3),(number)):(LuaDAffine3)

        --Creates an affine transform containing a 3D rotation around the x axis of
        --
        --`angle` (in radians).

        from_rotation_x : function(number):(LuaDAffine3)

        --Creates an affine transform containing a 3D rotation around the y axis of
        --
        --`angle` (in radians).

        from_rotation_y : function(number):(LuaDAffine3)

        --Creates an affine transform containing a 3D rotation around the z axis of
        --
        --`angle` (in radians).

        from_rotation_z : function(number):(LuaDAffine3)

        --Creates an affine transformation from the given 3D `translation`.

        from_translation : function(LuaDVec3):(LuaDAffine3)

        --Creates an affine transform from a 3x3 matrix (expressing scale, shear and
        --
        --rotation)

        from_mat3 : function(LuaDMat3):(LuaDAffine3)

        --Creates an affine transform from a 3x3 matrix (expressing scale, shear and rotation)
        --
        --and a translation vector.
        --
        --
        --
        --Equivalent to `DAffine3::from_translation(translation) * DAffine3::from_mat3(mat3)`

        from_mat3_translation : function((LuaDMat3),(LuaDVec3)):(LuaDAffine3)

        --Creates an affine transform from the given 3D `scale`, `rotation` and
        --
        --`translation`.
        --
        --
        --
        --Equivalent to `DAffine3::from_translation(translation) *
        --
        --DAffine3::from_quat(rotation) * DAffine3::from_scale(scale)`

        from_scale_rotation_translation : function((LuaDVec3),(LuaDQuat),(LuaDVec3)):(LuaDAffine3)

        --Creates an affine transform from the given 3D `rotation` and `translation`.
        --
        --
        --
        --Equivalent to `DAffine3::from_translation(translation) * DAffine3::from_quat(rotation)`

        from_rotation_translation : function((LuaDQuat),(LuaDVec3)):(LuaDAffine3)

        --The given `DMat4` must be an affine transform,
        --
        --i.e. contain no perspective transform.

        from_mat4 : function(LuaDMat4):(LuaDAffine3)

        --Creates a left-handed view transform using a camera position, an up direction, and a facing
        --
        --direction.
        --
        --
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.

        look_to_lh : function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDAffine3)

        --Creates a right-handed view transform using a camera position, an up direction, and a facing
        --
        --direction.
        --
        --
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.

        look_to_rh : function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDAffine3)

        --Creates a left-handed view transform using a camera position, an up direction, and a focal
        --
        --point.
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `up` is not normalized when `glam_assert` is enabled.

        look_at_lh : function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDAffine3)

        --Creates a right-handed view transform using a camera position, an up direction, and a focal
        --
        --point.
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `up` is not normalized when `glam_assert` is enabled.

        look_at_rh : function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDAffine3)

        __tostring : function(LuaDAffine3):(string)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaDAffine3Class

        userdata
    
    --Collection of static methods for [`LuaDAffine3`].

        --Creates an affine transform from three column vectors.

        from_cols : function((LuaDVec3),(LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDAffine3)

        --Creates an affine transform that changes scale.
        --
        --Note that if any scale is zero the transform will be non-invertible.

        from_scale : function(LuaDVec3):(LuaDAffine3)

        --Creates an affine transform from the given `rotation` quaternion.

        from_quat : function(LuaDQuat):(LuaDAffine3)

        --Creates an affine transform containing a 3D rotation around a normalized
        --
        --rotation `axis` of `angle` (in radians).

        from_axis_angle : function((LuaDVec3),(number)):(LuaDAffine3)

        --Creates an affine transform containing a 3D rotation around the x axis of
        --
        --`angle` (in radians).

        from_rotation_x : function(number):(LuaDAffine3)

        --Creates an affine transform containing a 3D rotation around the y axis of
        --
        --`angle` (in radians).

        from_rotation_y : function(number):(LuaDAffine3)

        --Creates an affine transform containing a 3D rotation around the z axis of
        --
        --`angle` (in radians).

        from_rotation_z : function(number):(LuaDAffine3)

        --Creates an affine transformation from the given 3D `translation`.

        from_translation : function(LuaDVec3):(LuaDAffine3)

        --Creates an affine transform from a 3x3 matrix (expressing scale, shear and
        --
        --rotation)

        from_mat3 : function(LuaDMat3):(LuaDAffine3)

        --Creates an affine transform from a 3x3 matrix (expressing scale, shear and rotation)
        --
        --and a translation vector.
        --
        --
        --
        --Equivalent to `DAffine3::from_translation(translation) * DAffine3::from_mat3(mat3)`

        from_mat3_translation : function((LuaDMat3),(LuaDVec3)):(LuaDAffine3)

        --Creates an affine transform from the given 3D `scale`, `rotation` and
        --
        --`translation`.
        --
        --
        --
        --Equivalent to `DAffine3::from_translation(translation) *
        --
        --DAffine3::from_quat(rotation) * DAffine3::from_scale(scale)`

        from_scale_rotation_translation : function((LuaDVec3),(LuaDQuat),(LuaDVec3)):(LuaDAffine3)

        --Creates an affine transform from the given 3D `rotation` and `translation`.
        --
        --
        --
        --Equivalent to `DAffine3::from_translation(translation) * DAffine3::from_quat(rotation)`

        from_rotation_translation : function((LuaDQuat),(LuaDVec3)):(LuaDAffine3)

        --The given `DMat4` must be an affine transform,
        --
        --i.e. contain no perspective transform.

        from_mat4 : function(LuaDMat4):(LuaDAffine3)

        --Creates a left-handed view transform using a camera position, an up direction, and a facing
        --
        --direction.
        --
        --
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.

        look_to_lh : function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDAffine3)

        --Creates a right-handed view transform using a camera position, an up direction, and a facing
        --
        --direction.
        --
        --
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.

        look_to_rh : function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDAffine3)

        --Creates a left-handed view transform using a camera position, an up direction, and a focal
        --
        --point.
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `up` is not normalized when `glam_assert` is enabled.

        look_at_lh : function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDAffine3)

        --Creates a right-handed view transform using a camera position, an up direction, and a focal
        --
        --point.
        --
        --For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `up` is not normalized when `glam_assert` is enabled.

        look_at_rh : function((LuaDVec3),(LuaDVec3),(LuaDVec3)):(LuaDAffine3)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaQuat

        userdata
    
    --

        --Returns the rotation axis scaled by the rotation in radians.

        to_scaled_axis : function(LuaQuat):(LuaVec3)

        --Returns the vector part of the quaternion.

        xyz : function(LuaQuat):(LuaVec3)

        --Returns the quaternion conjugate of `self`. For a unit quaternion the
        --
        --conjugate is also the inverse.

        conjugate : function(LuaQuat):(LuaQuat)

        --Returns the inverse of a normalized quaternion.
        --
        --
        --
        --Typically quaternion inverse returns the conjugate of a normalized quaternion.
        --
        --Because `self` is assumed to already be unit length this method *does not* normalize
        --
        --before returning the conjugate.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `self` is not normalized when `glam_assert` is enabled.

        inverse : function(LuaQuat):(LuaQuat)

        --Computes the dot product of `self` and `rhs`. The dot product is
        --
        --equal to the cosine of the angle between two quaternion rotations.

        dot : function(LuaQuat,LuaQuat):(number)

        --Computes the length of `self`.

        length : function(LuaQuat):(number)

        --Computes the squared length of `self`.
        --
        --
        --
        --This is generally faster than `length()` as it avoids a square
        --
        --root operation.

        length_squared : function(LuaQuat):(number)

        --Computes `1.0 / length()`.
        --
        --
        --
        --For valid results, `self` must _not_ be of length zero.

        length_recip : function(LuaQuat):(number)

        --Returns `self` normalized to length 1.0.
        --
        --
        --
        --For valid results, `self` must _not_ be of length zero.
        --
        --
        --
        --Panics
        --
        --
        --
        --Will panic if `self` is zero length when `glam_assert` is enabled.

        normalize : function(LuaQuat):(LuaQuat)

        --Returns `true` if, and only if, all elements are finite.
        --
        --If any element is either `NaN`, positive or negative infinity, this will return `false`.

        is_finite : function(LuaQuat):(boolean)

        is_nan : function(LuaQuat):(boolean)

        --Returns whether `self` of length `1.0` or not.
        --
        --
        --
        --Uses a precision threshold of `1e-6`.

        is_normalized : function(LuaQuat):(boolean)

        is_near_identity : function(LuaQuat):(boolean)

        --Returns the angle (in radians) for the minimal rotation
        --
        --for transforming this quaternion into another.
        --
        --
        --
        --Both quaternions must be normalized.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.

        angle_between : function(LuaQuat,LuaQuat):(number)

        --Returns true if the absolute difference of all elements between `self` and `rhs`
        --
        --is less than or equal to `max_abs_diff`.
        --
        --
        --
        --This can be used to compare if two quaternions contain similar elements. It works
        --
        --best when comparing with a known value. The `max_abs_diff` that should be used used
        --
        --depends on the values being compared against.
        --
        --
        --
        --For more see
        --
        --[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

        abs_diff_eq : function(LuaQuat,(LuaQuat),(number)):(boolean)

        --Performs a linear interpolation between `self` and `rhs` based on
        --
        --the value `s`.
        --
        --
        --
        --When `s` is `0.0`, the result will be equal to `self`.  When `s`
        --
        --is `1.0`, the result will be equal to `rhs`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.

        lerp : function(LuaQuat,(LuaQuat),(number)):(LuaQuat)

        --Performs a spherical linear interpolation between `self` and `end`
        --
        --based on the value `s`.
        --
        --
        --
        --When `s` is `0.0`, the result will be equal to `self`.  When `s`
        --
        --is `1.0`, the result will be equal to `end`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.

        slerp : function(LuaQuat,(LuaQuat),(number)):(LuaQuat)

        --Multiplies a quaternion and a 3D vector, returning the rotated vector.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `self` is not normalized when `glam_assert` is enabled.

        mul_vec3 : function(LuaQuat,LuaVec3):(LuaVec3)

        --Multiplies two quaternions. If they each represent a rotation, the result will
        --
        --represent the combined rotation.
        --
        --
        --
        --Note that due to floating point rounding the result may not be perfectly normalized.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.

        mul_quat : function(LuaQuat,LuaQuat):(LuaQuat)

        --Multiplies a quaternion and a 3D vector, returning the rotated vector.

        mul_vec3a : function(LuaQuat,LuaVec3A):(LuaVec3A)

        as_f64 : function(LuaQuat):(LuaDQuat)

        --Creates a new rotation quaternion.
        --
        --
        --
        --This should generally not be called manually unless you know what you are doing.
        --
        --Use one of the other constructors instead such as `identity` or `from_axis_angle`.
        --
        --
        --
        --`from_xyzw` is mostly used by unit tests and `serde` deserialization.
        --
        --
        --
        --# Preconditions
        --
        --
        --
        --This function does not check if the input is normalized, it is up to the user to
        --
        --provide normalized input or to normalized the resulting quaternion.

        from_xyzw : function((number),(number),(number),(number)):(LuaQuat)

        --Creates a new rotation quaternion from a 4D vector.
        --
        --
        --
        --# Preconditions
        --
        --
        --
        --This function does not check if the input is normalized, it is up to the user to
        --
        --provide normalized input or to normalized the resulting quaternion.

        from_vec4 : function(LuaVec4):(LuaQuat)

        --Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
        --
        --
        --
        --The axis must be a unit vector.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `axis` is not normalized when `glam_assert` is enabled.

        from_axis_angle : function((LuaVec3),(number)):(LuaQuat)

        --Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
        --
        --
        --
        --`from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.

        from_scaled_axis : function(LuaVec3):(LuaQuat)

        --Creates a quaternion from the `angle` (in radians) around the x axis.

        from_rotation_x : function(number):(LuaQuat)

        --Creates a quaternion from the `angle` (in radians) around the y axis.

        from_rotation_y : function(number):(LuaQuat)

        --Creates a quaternion from the `angle` (in radians) around the z axis.

        from_rotation_z : function(number):(LuaQuat)

        --Creates a quaternion from the given Euler rotation sequence and the angles (in radians).

        from_euler : function((LuaEulerRot),(number),(number),(number)):(LuaQuat)

        --Creates a quaternion from a 3x3 rotation matrix.

        from_mat3 : function(LuaMat3):(LuaQuat)

        --Creates a quaternion from a 3x3 SIMD aligned rotation matrix.

        from_mat3a : function(LuaMat3A):(LuaQuat)

        --Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.

        from_mat4 : function(LuaMat4):(LuaQuat)

        --Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
        --
        --plane spanned by the two vectors.  Will rotate at most 180 degrees.
        --
        --
        --
        --The inputs must be unit vectors.
        --
        --
        --
        --`from_rotation_arc(from, to) * from ≈ to`.
        --
        --
        --
        --For near-singular cases (from≈to and from≈-to) the current implementation
        --
        --is only accurate to about 0.001 (for `f32`).
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

        from_rotation_arc : function((LuaVec3),(LuaVec3)):(LuaQuat)

        --Gets the minimal rotation for transforming `from` to either `to` or `-to`.  This means
        --
        --that the resulting quaternion will rotate `from` so that it is colinear with `to`.
        --
        --
        --
        --The rotation is in the plane spanned by the two vectors.  Will rotate at most 90
        --
        --degrees.
        --
        --
        --
        --The inputs must be unit vectors.
        --
        --
        --
        --`to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

        from_rotation_arc_colinear : function((LuaVec3),(LuaVec3)):(LuaQuat)

        --Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
        --
        --around the z axis. Will rotate at most 180 degrees.
        --
        --
        --
        --The inputs must be unit vectors.
        --
        --
        --
        --`from_rotation_arc_2d(from, to) * from ≈ to`.
        --
        --
        --
        --For near-singular cases (from≈to and from≈-to) the current implementation
        --
        --is only accurate to about 0.001 (for `f32`).
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

        from_rotation_arc_2d : function((LuaVec2),(LuaVec2)):(LuaQuat)

        --Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.

        from_affine3 : function(LuaAffine3A):(LuaQuat)

        __tostring : function(LuaQuat):(string)

        __unm : function(LuaQuat):(LuaQuat)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaQuatClass

        userdata
    
    --Collection of static methods for [`LuaQuat`].

        --Creates a new rotation quaternion.
        --
        --
        --
        --This should generally not be called manually unless you know what you are doing.
        --
        --Use one of the other constructors instead such as `identity` or `from_axis_angle`.
        --
        --
        --
        --`from_xyzw` is mostly used by unit tests and `serde` deserialization.
        --
        --
        --
        --# Preconditions
        --
        --
        --
        --This function does not check if the input is normalized, it is up to the user to
        --
        --provide normalized input or to normalized the resulting quaternion.

        from_xyzw : function((number),(number),(number),(number)):(LuaQuat)

        --Creates a new rotation quaternion from a 4D vector.
        --
        --
        --
        --# Preconditions
        --
        --
        --
        --This function does not check if the input is normalized, it is up to the user to
        --
        --provide normalized input or to normalized the resulting quaternion.

        from_vec4 : function(LuaVec4):(LuaQuat)

        --Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
        --
        --
        --
        --The axis must be a unit vector.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `axis` is not normalized when `glam_assert` is enabled.

        from_axis_angle : function((LuaVec3),(number)):(LuaQuat)

        --Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
        --
        --
        --
        --`from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.

        from_scaled_axis : function(LuaVec3):(LuaQuat)

        --Creates a quaternion from the `angle` (in radians) around the x axis.

        from_rotation_x : function(number):(LuaQuat)

        --Creates a quaternion from the `angle` (in radians) around the y axis.

        from_rotation_y : function(number):(LuaQuat)

        --Creates a quaternion from the `angle` (in radians) around the z axis.

        from_rotation_z : function(number):(LuaQuat)

        --Creates a quaternion from the given Euler rotation sequence and the angles (in radians).

        from_euler : function((LuaEulerRot),(number),(number),(number)):(LuaQuat)

        --Creates a quaternion from a 3x3 rotation matrix.

        from_mat3 : function(LuaMat3):(LuaQuat)

        --Creates a quaternion from a 3x3 SIMD aligned rotation matrix.

        from_mat3a : function(LuaMat3A):(LuaQuat)

        --Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.

        from_mat4 : function(LuaMat4):(LuaQuat)

        --Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
        --
        --plane spanned by the two vectors.  Will rotate at most 180 degrees.
        --
        --
        --
        --The inputs must be unit vectors.
        --
        --
        --
        --`from_rotation_arc(from, to) * from ≈ to`.
        --
        --
        --
        --For near-singular cases (from≈to and from≈-to) the current implementation
        --
        --is only accurate to about 0.001 (for `f32`).
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

        from_rotation_arc : function((LuaVec3),(LuaVec3)):(LuaQuat)

        --Gets the minimal rotation for transforming `from` to either `to` or `-to`.  This means
        --
        --that the resulting quaternion will rotate `from` so that it is colinear with `to`.
        --
        --
        --
        --The rotation is in the plane spanned by the two vectors.  Will rotate at most 90
        --
        --degrees.
        --
        --
        --
        --The inputs must be unit vectors.
        --
        --
        --
        --`to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

        from_rotation_arc_colinear : function((LuaVec3),(LuaVec3)):(LuaQuat)

        --Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
        --
        --around the z axis. Will rotate at most 180 degrees.
        --
        --
        --
        --The inputs must be unit vectors.
        --
        --
        --
        --`from_rotation_arc_2d(from, to) * from ≈ to`.
        --
        --
        --
        --For near-singular cases (from≈to and from≈-to) the current implementation
        --
        --is only accurate to about 0.001 (for `f32`).
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

        from_rotation_arc_2d : function((LuaVec2),(LuaVec2)):(LuaQuat)

        --Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.

        from_affine3 : function(LuaAffine3A):(LuaQuat)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaDQuat

        userdata
    
    --

        x : number

        y : number

        z : number

        w : number

        --Returns the rotation axis scaled by the rotation in radians.

        to_scaled_axis : function(LuaDQuat):(LuaDVec3)

        --Returns the vector part of the quaternion.

        xyz : function(LuaDQuat):(LuaDVec3)

        --Returns the quaternion conjugate of `self`. For a unit quaternion the
        --
        --conjugate is also the inverse.

        conjugate : function(LuaDQuat):(LuaDQuat)

        --Returns the inverse of a normalized quaternion.
        --
        --
        --
        --Typically quaternion inverse returns the conjugate of a normalized quaternion.
        --
        --Because `self` is assumed to already be unit length this method *does not* normalize
        --
        --before returning the conjugate.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `self` is not normalized when `glam_assert` is enabled.

        inverse : function(LuaDQuat):(LuaDQuat)

        --Computes the dot product of `self` and `rhs`. The dot product is
        --
        --equal to the cosine of the angle between two quaternion rotations.

        dot : function(LuaDQuat,LuaDQuat):(number)

        --Computes the length of `self`.

        length : function(LuaDQuat):(number)

        --Computes the squared length of `self`.
        --
        --
        --
        --This is generally faster than `length()` as it avoids a square
        --
        --root operation.

        length_squared : function(LuaDQuat):(number)

        --Computes `1.0 / length()`.
        --
        --
        --
        --For valid results, `self` must _not_ be of length zero.

        length_recip : function(LuaDQuat):(number)

        --Returns `self` normalized to length 1.0.
        --
        --
        --
        --For valid results, `self` must _not_ be of length zero.
        --
        --
        --
        --Panics
        --
        --
        --
        --Will panic if `self` is zero length when `glam_assert` is enabled.

        normalize : function(LuaDQuat):(LuaDQuat)

        --Returns `true` if, and only if, all elements are finite.
        --
        --If any element is either `NaN`, positive or negative infinity, this will return `false`.

        is_finite : function(LuaDQuat):(boolean)

        is_nan : function(LuaDQuat):(boolean)

        --Returns whether `self` of length `1.0` or not.
        --
        --
        --
        --Uses a precision threshold of `1e-6`.

        is_normalized : function(LuaDQuat):(boolean)

        is_near_identity : function(LuaDQuat):(boolean)

        --Returns the angle (in radians) for the minimal rotation
        --
        --for transforming this quaternion into another.
        --
        --
        --
        --Both quaternions must be normalized.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.

        angle_between : function(LuaDQuat,LuaDQuat):(number)

        --Returns true if the absolute difference of all elements between `self` and `rhs`
        --
        --is less than or equal to `max_abs_diff`.
        --
        --
        --
        --This can be used to compare if two quaternions contain similar elements. It works
        --
        --best when comparing with a known value. The `max_abs_diff` that should be used used
        --
        --depends on the values being compared against.
        --
        --
        --
        --For more see
        --
        --[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

        abs_diff_eq : function(LuaDQuat,(LuaDQuat),(number)):(boolean)

        --Performs a linear interpolation between `self` and `rhs` based on
        --
        --the value `s`.
        --
        --
        --
        --When `s` is `0.0`, the result will be equal to `self`.  When `s`
        --
        --is `1.0`, the result will be equal to `rhs`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.

        lerp : function(LuaDQuat,(LuaDQuat),(number)):(LuaDQuat)

        --Performs a spherical linear interpolation between `self` and `end`
        --
        --based on the value `s`.
        --
        --
        --
        --When `s` is `0.0`, the result will be equal to `self`.  When `s`
        --
        --is `1.0`, the result will be equal to `end`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.

        slerp : function(LuaDQuat,(LuaDQuat),(number)):(LuaDQuat)

        --Multiplies a quaternion and a 3D vector, returning the rotated vector.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `self` is not normalized when `glam_assert` is enabled.

        mul_vec3 : function(LuaDQuat,LuaDVec3):(LuaDVec3)

        --Multiplies two quaternions. If they each represent a rotation, the result will
        --
        --represent the combined rotation.
        --
        --
        --
        --Note that due to floating point rounding the result may not be perfectly normalized.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.

        mul_quat : function(LuaDQuat,LuaDQuat):(LuaDQuat)

        as_f32 : function(LuaDQuat):(LuaQuat)

        --Creates a new rotation quaternion.
        --
        --
        --
        --This should generally not be called manually unless you know what you are doing.
        --
        --Use one of the other constructors instead such as `identity` or `from_axis_angle`.
        --
        --
        --
        --`from_xyzw` is mostly used by unit tests and `serde` deserialization.
        --
        --
        --
        --# Preconditions
        --
        --
        --
        --This function does not check if the input is normalized, it is up to the user to
        --
        --provide normalized input or to normalized the resulting quaternion.

        from_xyzw : function((number),(number),(number),(number)):(LuaDQuat)

        --Creates a new rotation quaternion from a 4D vector.
        --
        --
        --
        --# Preconditions
        --
        --
        --
        --This function does not check if the input is normalized, it is up to the user to
        --
        --provide normalized input or to normalized the resulting quaternion.

        from_vec4 : function(LuaDVec4):(LuaDQuat)

        --Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
        --
        --
        --
        --The axis must be a unit vector.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `axis` is not normalized when `glam_assert` is enabled.

        from_axis_angle : function((LuaDVec3),(number)):(LuaDQuat)

        --Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
        --
        --
        --
        --`from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.

        from_scaled_axis : function(LuaDVec3):(LuaDQuat)

        --Creates a quaternion from the `angle` (in radians) around the x axis.

        from_rotation_x : function(number):(LuaDQuat)

        --Creates a quaternion from the `angle` (in radians) around the y axis.

        from_rotation_y : function(number):(LuaDQuat)

        --Creates a quaternion from the `angle` (in radians) around the z axis.

        from_rotation_z : function(number):(LuaDQuat)

        --Creates a quaternion from the given Euler rotation sequence and the angles (in radians).

        from_euler : function((LuaEulerRot),(number),(number),(number)):(LuaDQuat)

        --Creates a quaternion from a 3x3 rotation matrix.

        from_mat3 : function(LuaDMat3):(LuaDQuat)

        --Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.

        from_mat4 : function(LuaDMat4):(LuaDQuat)

        --Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
        --
        --plane spanned by the two vectors.  Will rotate at most 180 degrees.
        --
        --
        --
        --The inputs must be unit vectors.
        --
        --
        --
        --`from_rotation_arc(from, to) * from ≈ to`.
        --
        --
        --
        --For near-singular cases (from≈to and from≈-to) the current implementation
        --
        --is only accurate to about 0.001 (for `f32`).
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

        from_rotation_arc : function((LuaDVec3),(LuaDVec3)):(LuaDQuat)

        --Gets the minimal rotation for transforming `from` to either `to` or `-to`.  This means
        --
        --that the resulting quaternion will rotate `from` so that it is colinear with `to`.
        --
        --
        --
        --The rotation is in the plane spanned by the two vectors.  Will rotate at most 90
        --
        --degrees.
        --
        --
        --
        --The inputs must be unit vectors.
        --
        --
        --
        --`to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

        from_rotation_arc_colinear : function((LuaDVec3),(LuaDVec3)):(LuaDQuat)

        --Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
        --
        --around the z axis. Will rotate at most 180 degrees.
        --
        --
        --
        --The inputs must be unit vectors.
        --
        --
        --
        --`from_rotation_arc_2d(from, to) * from ≈ to`.
        --
        --
        --
        --For near-singular cases (from≈to and from≈-to) the current implementation
        --
        --is only accurate to about 0.001 (for `f32`).
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

        from_rotation_arc_2d : function((LuaDVec2),(LuaDVec2)):(LuaDQuat)

        --Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.

        from_affine3 : function(LuaDAffine3):(LuaDQuat)

        __tostring : function(LuaDQuat):(string)

        __unm : function(LuaDQuat):(LuaDQuat)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaDQuatClass

        userdata
    
    --Collection of static methods for [`LuaDQuat`].

        --Creates a new rotation quaternion.
        --
        --
        --
        --This should generally not be called manually unless you know what you are doing.
        --
        --Use one of the other constructors instead such as `identity` or `from_axis_angle`.
        --
        --
        --
        --`from_xyzw` is mostly used by unit tests and `serde` deserialization.
        --
        --
        --
        --# Preconditions
        --
        --
        --
        --This function does not check if the input is normalized, it is up to the user to
        --
        --provide normalized input or to normalized the resulting quaternion.

        from_xyzw : function((number),(number),(number),(number)):(LuaDQuat)

        --Creates a new rotation quaternion from a 4D vector.
        --
        --
        --
        --# Preconditions
        --
        --
        --
        --This function does not check if the input is normalized, it is up to the user to
        --
        --provide normalized input or to normalized the resulting quaternion.

        from_vec4 : function(LuaDVec4):(LuaDQuat)

        --Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
        --
        --
        --
        --The axis must be a unit vector.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `axis` is not normalized when `glam_assert` is enabled.

        from_axis_angle : function((LuaDVec3),(number)):(LuaDQuat)

        --Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
        --
        --
        --
        --`from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.

        from_scaled_axis : function(LuaDVec3):(LuaDQuat)

        --Creates a quaternion from the `angle` (in radians) around the x axis.

        from_rotation_x : function(number):(LuaDQuat)

        --Creates a quaternion from the `angle` (in radians) around the y axis.

        from_rotation_y : function(number):(LuaDQuat)

        --Creates a quaternion from the `angle` (in radians) around the z axis.

        from_rotation_z : function(number):(LuaDQuat)

        --Creates a quaternion from the given Euler rotation sequence and the angles (in radians).

        from_euler : function((LuaEulerRot),(number),(number),(number)):(LuaDQuat)

        --Creates a quaternion from a 3x3 rotation matrix.

        from_mat3 : function(LuaDMat3):(LuaDQuat)

        --Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.

        from_mat4 : function(LuaDMat4):(LuaDQuat)

        --Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
        --
        --plane spanned by the two vectors.  Will rotate at most 180 degrees.
        --
        --
        --
        --The inputs must be unit vectors.
        --
        --
        --
        --`from_rotation_arc(from, to) * from ≈ to`.
        --
        --
        --
        --For near-singular cases (from≈to and from≈-to) the current implementation
        --
        --is only accurate to about 0.001 (for `f32`).
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

        from_rotation_arc : function((LuaDVec3),(LuaDVec3)):(LuaDQuat)

        --Gets the minimal rotation for transforming `from` to either `to` or `-to`.  This means
        --
        --that the resulting quaternion will rotate `from` so that it is colinear with `to`.
        --
        --
        --
        --The rotation is in the plane spanned by the two vectors.  Will rotate at most 90
        --
        --degrees.
        --
        --
        --
        --The inputs must be unit vectors.
        --
        --
        --
        --`to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

        from_rotation_arc_colinear : function((LuaDVec3),(LuaDVec3)):(LuaDQuat)

        --Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
        --
        --around the z axis. Will rotate at most 180 degrees.
        --
        --
        --
        --The inputs must be unit vectors.
        --
        --
        --
        --`from_rotation_arc_2d(from, to) * from ≈ to`.
        --
        --
        --
        --For near-singular cases (from≈to and from≈-to) the current implementation
        --
        --is only accurate to about 0.001 (for `f32`).
        --
        --
        --
        --# Panics
        --
        --
        --
        --Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

        from_rotation_arc_2d : function((LuaDVec2),(LuaDVec2)):(LuaDQuat)

        --Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.

        from_affine3 : function(LuaDAffine3):(LuaDQuat)

        __add : function((any),(any)):(any)

        __sub : function((any),(any)):(any)

        __div : function((any),(any)):(any)

        __mul : function((any),(any)):(any)

    end
    
    
    record LuaEulerRot

        userdata
    
    --

        __tostring : function(LuaEulerRot):(string)

    end
    
    
    record LuaRect

        userdata
    
    --

        -- The minimum corner point of the rect.

        min : LuaVec2

        -- The maximum corner point of the rect.

        max : LuaVec2

        --Check if the rectangle is empty.
        --
        --
        --
        --# Examples
        --
        --
        --
        --```rust
        --
        --# use bevy_math::{Rect, Vec2};
        --
        --let r = Rect::from_corners(Vec2::ZERO, Vec2::new(0., 1.)); // w=0 h=1
        --
        --assert!(r.is_empty());
        --
        --```

        is_empty : function(LuaRect):(boolean)

        --Rectangle width (max.x - min.x).
        --
        --
        --
        --# Examples
        --
        --
        --
        --```rust
        --
        --# use bevy_math::Rect;
        --
        --let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
        --
        --assert!((r.width() - 5.).abs() <= 1e-5);
        --
        --```

        width : function(LuaRect):(number)

        --Rectangle height (max.y - min.y).
        --
        --
        --
        --# Examples
        --
        --
        --
        --```rust
        --
        --# use bevy_math::Rect;
        --
        --let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
        --
        --assert!((r.height() - 1.).abs() <= 1e-5);
        --
        --```

        height : function(LuaRect):(number)

        --Rectangle size.
        --
        --
        --
        --# Examples
        --
        --
        --
        --```rust
        --
        --# use bevy_math::{Rect, Vec2};
        --
        --let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
        --
        --assert!(r.size().abs_diff_eq(Vec2::new(5., 1.), 1e-5));
        --
        --```

        size : function(LuaRect):(LuaVec2)

        --Rectangle half-size.
        --
        --
        --
        --# Examples
        --
        --
        --
        --```rust
        --
        --# use bevy_math::{Rect, Vec2};
        --
        --let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
        --
        --assert!(r.half_size().abs_diff_eq(Vec2::new(2.5, 0.5), 1e-5));
        --
        --```

        half_size : function(LuaRect):(LuaVec2)

        --The center point of the rectangle.
        --
        --
        --
        --# Examples
        --
        --
        --
        --```rust
        --
        --# use bevy_math::{Rect, Vec2};
        --
        --let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
        --
        --assert!(r.center().abs_diff_eq(Vec2::new(2.5, 0.5), 1e-5));
        --
        --```

        center : function(LuaRect):(LuaVec2)

        --Check if a point lies within this rectangle, inclusive of its edges.
        --
        --
        --
        --# Examples
        --
        --
        --
        --```rust
        --
        --# use bevy_math::Rect;
        --
        --let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
        --
        --assert!(r.contains(r.center()));
        --
        --assert!(r.contains(r.min));
        --
        --assert!(r.contains(r.max));
        --
        --```

        contains : function(LuaRect,LuaVec2):(boolean)

        --Build a new rectangle formed of the union of this rectangle and another rectangle.
        --
        --
        --
        --The union is the smallest rectangle enclosing both rectangles.
        --
        --
        --
        --# Examples
        --
        --
        --
        --```rust
        --
        --# use bevy_math::{Rect, Vec2};
        --
        --let r1 = Rect::new(0., 0., 5., 1.); // w=5 h=1
        --
        --let r2 = Rect::new(1., -1., 3., 3.); // w=2 h=4
        --
        --let r = r1.union(r2);
        --
        --assert!(r.min.abs_diff_eq(Vec2::new(0., -1.), 1e-5));
        --
        --assert!(r.max.abs_diff_eq(Vec2::new(5., 3.), 1e-5));
        --
        --```

        union : function(LuaRect,LuaRect):(LuaRect)

        --Build a new rectangle formed of the union of this rectangle and a point.
        --
        --
        --
        --The union is the smallest rectangle enclosing both the rectangle and the point. If the
        --
        --point is already inside the rectangle, this method returns a copy of the rectangle.
        --
        --
        --
        --# Examples
        --
        --
        --
        --```rust
        --
        --# use bevy_math::{Rect, Vec2};
        --
        --let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
        --
        --let u = r.union_point(Vec2::new(3., 6.));
        --
        --assert!(u.min.abs_diff_eq(Vec2::ZERO, 1e-5));
        --
        --assert!(u.max.abs_diff_eq(Vec2::new(5., 6.), 1e-5));
        --
        --```

        union_point : function(LuaRect,LuaVec2):(LuaRect)

        --Build a new rectangle formed of the intersection of this rectangle and another rectangle.
        --
        --
        --
        --The intersection is the largest rectangle enclosed in both rectangles. If the intersection
        --
        --is empty, this method returns an empty rectangle ([`Rect::is_empty()`] returns `true`), but
        --
        --the actual values of [`Rect::min`] and [`Rect::max`] are implementation-dependent.
        --
        --
        --
        --# Examples
        --
        --
        --
        --```rust
        --
        --# use bevy_math::{Rect, Vec2};
        --
        --let r1 = Rect::new(0., 0., 5., 1.); // w=5 h=1
        --
        --let r2 = Rect::new(1., -1., 3., 3.); // w=2 h=4
        --
        --let r = r1.intersect(r2);
        --
        --assert!(r.min.abs_diff_eq(Vec2::new(1., 0.), 1e-5));
        --
        --assert!(r.max.abs_diff_eq(Vec2::new(3., 1.), 1e-5));
        --
        --```

        intersect : function(LuaRect,LuaRect):(LuaRect)

        --Create a new rectangle with a constant inset.
        --
        --
        --
        --The inset is the extra border on all sides. A positive inset produces a larger rectangle,
        --
        --while a negative inset is allowed and produces a smaller rectangle. If the inset is negative
        --
        --and its absolute value is larger than the rectangle half-size, the created rectangle is empty.
        --
        --
        --
        --# Examples
        --
        --
        --
        --```rust
        --
        --# use bevy_math::{Rect, Vec2};
        --
        --let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
        --
        --let r2 = r.inset(3.); // w=11 h=7
        --
        --assert!(r2.min.abs_diff_eq(Vec2::splat(-3.), 1e-5));
        --
        --assert!(r2.max.abs_diff_eq(Vec2::new(8., 4.), 1e-5));
        --
        --
        --
        --let r = Rect::new(0., -1., 6., 7.); // w=6 h=8
        --
        --let r2 = r.inset(-2.); // w=11 h=7
        --
        --assert!(r2.min.abs_diff_eq(Vec2::new(2., 1.), 1e-5));
        --
        --assert!(r2.max.abs_diff_eq(Vec2::new(4., 5.), 1e-5));
        --
        --```

        inset : function(LuaRect,number):(LuaRect)

        --Create a new rectangle from two corner points.
        --
        --
        --
        --The two points do not need to be the minimum and/or maximum corners.
        --
        --They only need to be two opposite corners.
        --
        --
        --
        --# Examples
        --
        --
        --
        --```rust
        --
        --# use bevy_math::Rect;
        --
        --let r = Rect::new(0., 4., 10., 6.); // w=10 h=2
        --
        --let r = Rect::new(2., 3., 5., -1.); // w=3 h=4
        --
        --```

        new : function((number),(number),(number),(number)):(LuaRect)

        --Create a new rectangle from two corner points.
        --
        --
        --
        --The two points do not need to be the minimum and/or maximum corners.
        --
        --They only need to be two opposite corners.
        --
        --
        --
        --# Examples
        --
        --
        --
        --```rust
        --
        --# use bevy_math::{Rect, Vec2};
        --
        --let r = Rect::from_corners(Vec2::ZERO, Vec2::ONE); // w=1 h=1
        --
        --let r = Rect::from_corners(Vec2::ONE, Vec2::ZERO); // w=1 h=1
        --
        --```

        from_corners : function((LuaVec2),(LuaVec2)):(LuaRect)

        --Create a new rectangle from its center and size.
        --
        --
        --
        --# Panics
        --
        --
        --
        --This method panics if any of the components of the size is negative.
        --
        --
        --
        --# Examples
        --
        --
        --
        --```rust
        --
        --# use bevy_math::{Rect, Vec2};
        --
        --let r = Rect::from_center_size(Vec2::ZERO, Vec2::ONE); // w=1 h=1
        --
        --assert!(r.min.abs_diff_eq(Vec2::splat(-0.5), 1e-5));
        --
        --assert!(r.max.abs_diff_eq(Vec2::splat(0.5), 1e-5));
        --
        --```

        from_center_size : function((LuaVec2),(LuaVec2)):(LuaRect)

        --Create a new rectangle from its center and half-size.
        --
        --
        --
        --# Panics
        --
        --
        --
        --This method panics if any of the components of the half-size is negative.
        --
        --
        --
        --# Examples
        --
        --
        --
        --```rust
        --
        --# use bevy_math::{Rect, Vec2};
        --
        --let r = Rect::from_center_half_size(Vec2::ZERO, Vec2::ONE); // w=2 h=2
        --
        --assert!(r.min.abs_diff_eq(Vec2::splat(-1.), 1e-5));
        --
        --assert!(r.max.abs_diff_eq(Vec2::splat(1.), 1e-5));
        --
        --```

        from_center_half_size : function((LuaVec2),(LuaVec2)):(LuaRect)

        __tostring : function(LuaRect):(string)

    end
    
    
    record LuaRectClass

        userdata
    
    --Collection of static methods for [`LuaRect`].

        --Create a new rectangle from two corner points.
        --
        --
        --
        --The two points do not need to be the minimum and/or maximum corners.
        --
        --They only need to be two opposite corners.
        --
        --
        --
        --# Examples
        --
        --
        --
        --```rust
        --
        --# use bevy_math::Rect;
        --
        --let r = Rect::new(0., 4., 10., 6.); // w=10 h=2
        --
        --let r = Rect::new(2., 3., 5., -1.); // w=3 h=4
        --
        --```

        new : function((number),(number),(number),(number)):(LuaRect)

        --Create a new rectangle from two corner points.
        --
        --
        --
        --The two points do not need to be the minimum and/or maximum corners.
        --
        --They only need to be two opposite corners.
        --
        --
        --
        --# Examples
        --
        --
        --
        --```rust
        --
        --# use bevy_math::{Rect, Vec2};
        --
        --let r = Rect::from_corners(Vec2::ZERO, Vec2::ONE); // w=1 h=1
        --
        --let r = Rect::from_corners(Vec2::ONE, Vec2::ZERO); // w=1 h=1
        --
        --```

        from_corners : function((LuaVec2),(LuaVec2)):(LuaRect)

        --Create a new rectangle from its center and size.
        --
        --
        --
        --# Panics
        --
        --
        --
        --This method panics if any of the components of the size is negative.
        --
        --
        --
        --# Examples
        --
        --
        --
        --```rust
        --
        --# use bevy_math::{Rect, Vec2};
        --
        --let r = Rect::from_center_size(Vec2::ZERO, Vec2::ONE); // w=1 h=1
        --
        --assert!(r.min.abs_diff_eq(Vec2::splat(-0.5), 1e-5));
        --
        --assert!(r.max.abs_diff_eq(Vec2::splat(0.5), 1e-5));
        --
        --```

        from_center_size : function((LuaVec2),(LuaVec2)):(LuaRect)

        --Create a new rectangle from its center and half-size.
        --
        --
        --
        --# Panics
        --
        --
        --
        --This method panics if any of the components of the half-size is negative.
        --
        --
        --
        --# Examples
        --
        --
        --
        --```rust
        --
        --# use bevy_math::{Rect, Vec2};
        --
        --let r = Rect::from_center_half_size(Vec2::ZERO, Vec2::ONE); // w=2 h=2
        --
        --assert!(r.min.abs_diff_eq(Vec2::splat(-1.), 1e-5));
        --
        --assert!(r.max.abs_diff_eq(Vec2::splat(1.), 1e-5));
        --
        --```

        from_center_half_size : function((LuaVec2),(LuaVec2)):(LuaRect)

    end
    
    
    record ReflectedValue

        userdata
    
    --This type represents a generic reflected value.
        --
        --If you know the reflected value converts to a LuaType (via LuaProxyable), use the `as` operator to convert to said type.
        --
        --

        __tostring : function(ReflectedValue):(string)

        __index : function(ReflectedValue,any):(ReflectedValue)

        __newindex : function(ReflectedValue,(any),(any)):()

    end
    
    
    record LuaWorld

        userdata
    
    --Represents the bevy world all scripts live in.
        --
        --Provides ways to interact with and modify the world.
        --
        --

        --Retrieves type information given either a short (`MyType`) or fully qualified rust type name (`MyModule::MyType`).
        --
        --Returns `nil` if no such type exists or if one wasn't registered on the rust side.
        --
        --
        --
        --
        --This is used extensively in [`LuaWorld`]

        get_type_by_name : function(LuaWorld,string):(LuaTypeRegistration)

        --Inserts a component of the given type to the given entity by instantiating a default version of it.
        --
        --The component can then be modified using field access.

        add_default_component : function(LuaWorld,(LuaEntity),(LuaTypeRegistration)):(ReflectedValue)

        --Retrieves a component of the given type from the given entity.
        --
        --If such a component does not exist returns `nil`.

        get_component : function(LuaWorld,(LuaEntity),(LuaTypeRegistration)):(ReflectedValue)

        --Returns `true` if the given entity contains a component of the given type.

        has_component : function(LuaWorld,(LuaEntity),(LuaTypeRegistration)):(boolean)

        --Retrieves a resource of the given type from the world.
        --
        --If such a resource does not exist returns `nil`.

        get_resource : function(LuaWorld,LuaTypeRegistration):(ReflectedValue)

        --Removes the given resource from the world, if one doesn't exist it does nothing.

        remove_resource : function(LuaWorld,LuaTypeRegistration):()

        --Returns `true` if the world contains a resource of the given type.

        has_resource : function(LuaWorld,LuaTypeRegistration):(boolean)

        --Retrieves children entities of the parent entity if it has any.

        get_children : function(LuaWorld,LuaEntity):({LuaEntity})

        --Retrieves the parent entity of the given entity if it has any.

        get_parent : function(LuaWorld,LuaEntity):(LuaEntity)

        --Attaches children entities to the given parent entity.

        push_children : function(LuaWorld,(LuaEntity),({LuaEntity})):()

        --Removes children entities from the given parent entity.

        remove_children : function(LuaWorld,(LuaEntity),({LuaEntity})):()

        --Removes child entity from the given parent entity.

        remove_child : function(LuaWorld,(LuaEntity),(LuaEntity)):()

        --Inserts children entities to the given parent entity at the given index.

        insert_children : function(LuaWorld,(LuaEntity),(integer),({LuaEntity})):()

        --Inserts child entity to the given parent entity at the given index.

        insert_child : function(LuaWorld,(LuaEntity),(integer),(LuaEntity)):()

        --Despawns the given entity's children recursively

        despawn_children_recursive : function(LuaWorld,LuaEntity):()

        --Despawns the given entity and the entity's children recursively

        despawn_recursive : function(LuaWorld,LuaEntity):()

        --Spawns a new entity and returns its Entity ID

        spawn : function(LuaWorld):(LuaEntity)

        --Despawns the given entity if it exists, returns true if deletion was successfull

        despawn : function(LuaWorld,LuaEntity):(boolean)

        --Removes the given component from the given entity, does nothing if it doesn't exist on the entity.

        remove_component : function(LuaWorld,(LuaEntity),(LuaTypeRegistration)):()

        --Attaches child entity to the given parent entity.

        push_child : function(LuaWorld,(LuaEntity),(LuaEntity)):()

        __tostring : function(LuaWorld):(string)

    end
    
    
    record LuaWorldClass

        userdata
    
    --Collection of static methods for [`LuaWorld`].

    end
    
    
    record LuaScriptData

        userdata
    
    --

        --The unique ID of this script

        sid : integer

        __tostring : function(LuaScriptData):(string)

    end
    
    
    record LuaScriptDataClass

        userdata
    
    --Collection of static methods for [`LuaScriptData`].

    end
    
    
    record LuaTypeRegistration

        userdata
    
    --An object representing an existing and registered rust type.
        --
        --Can be obtained via [`LuaWorld::get_type_by_name`].
        --
        --

        --The [short name](https://docs.rs/bevy/latest/bevy/reflect/struct.TypeRegistration.html#method.get_short_name) of a type

        short_name : string

        --The full name of the type

        type_name : string

    end
    
    
    record LuaVec<T>

        userdata
    
    --A reference to the Vec<T> Rust type.
        --
        --All indexing begins at 1.
        --
        --

        to_table : function(LuaVec<T>):({any : any } )

        push : function(LuaVec<T>,any):()

        pop : function(LuaVec<T>):(any)

        clear : function(LuaVec<T>):()

        insert : function(LuaVec<T>,(integer),(any)):()

        remove : function(LuaVec<T>,integer):(any)

        __tostring : function(LuaVec<T>):(string)

        __index : function(LuaVec<T>,integer):(ReflectedValue)

        __len : function(LuaVec<T>):(integer)

        __newindex : function(LuaVec<T>,(integer),(any)):()

    end
    
    
    record LogModule

        userdata
    
    --logging api backed by the `tracing` crate
        --
        --

        --Log a message at the TRACE level.

        trace : function(string):()

        --Log a message at the DEBUG level.

        debug : function(string):()

        --Log a message at the INFO level.

        info : function(string):()

        --Log a message at the WARN level.

        warn : function(string):()

        --Log a message at the ERROR level.

        error : function(string):()

    end
    
end

    --

    global Overflow : BevyAPI.LuaOverflowClass

    --

    global UiImage : BevyAPI.LuaUiImageClass

    --

    global Name : BevyAPI.LuaNameClass

    --

    global Text : BevyAPI.LuaTextClass

    --

    global TextSection : BevyAPI.LuaTextSectionClass

    --

    global Stopwatch : BevyAPI.LuaStopwatchClass

    --

    global Timer : BevyAPI.LuaTimerClass

    --

    global Entity : BevyAPI.LuaEntityClass

    --

    global Transform : BevyAPI.LuaTransformClass

    --

    global TextureAtlasSprite : BevyAPI.LuaTextureAtlasSpriteClass

    --

    global RenderLayers : BevyAPI.LuaRenderLayersClass

    --

    global Color : BevyAPI.LuaColorClass

    --

    global Aabb : BevyAPI.LuaAabbClass

    --

    global Frustum : BevyAPI.LuaFrustumClass

    --

    global CameraRenderGraph : BevyAPI.LuaCameraRenderGraphClass

    --

    global HandleId : BevyAPI.LuaHandleIdClass

    --

    global Vec2 : BevyAPI.LuaVec2Class

    --

    global Vec3 : BevyAPI.LuaVec3Class

    --

    global Vec3A : BevyAPI.LuaVec3AClass

    --

    global Vec4 : BevyAPI.LuaVec4Class

    --

    global BVec2 : BevyAPI.LuaBVec2Class

    --

    global BVec3 : BevyAPI.LuaBVec3Class

    --

    global BVec4 : BevyAPI.LuaBVec4Class

    --

    global BVec3A : BevyAPI.LuaBVec3AClass

    --

    global BVec4A : BevyAPI.LuaBVec4AClass

    --

    global DVec2 : BevyAPI.LuaDVec2Class

    --

    global DVec3 : BevyAPI.LuaDVec3Class

    --

    global DVec4 : BevyAPI.LuaDVec4Class

    --

    global IVec2 : BevyAPI.LuaIVec2Class

    --

    global IVec3 : BevyAPI.LuaIVec3Class

    --

    global IVec4 : BevyAPI.LuaIVec4Class

    --

    global UVec2 : BevyAPI.LuaUVec2Class

    --

    global UVec3 : BevyAPI.LuaUVec3Class

    --

    global UVec4 : BevyAPI.LuaUVec4Class

    --

    global Mat3 : BevyAPI.LuaMat3Class

    --

    global Mat2 : BevyAPI.LuaMat2Class

    --

    global Mat3A : BevyAPI.LuaMat3AClass

    --

    global Mat4 : BevyAPI.LuaMat4Class

    --

    global DMat2 : BevyAPI.LuaDMat2Class

    --

    global DMat3 : BevyAPI.LuaDMat3Class

    --

    global DMat4 : BevyAPI.LuaDMat4Class

    --

    global Affine2 : BevyAPI.LuaAffine2Class

    --

    global Affine3A : BevyAPI.LuaAffine3AClass

    --

    global DAffine2 : BevyAPI.LuaDAffine2Class

    --

    global DAffine3 : BevyAPI.LuaDAffine3Class

    --

    global Quat : BevyAPI.LuaQuatClass

    --

    global DQuat : BevyAPI.LuaDQuatClass

    --

    global Rect : BevyAPI.LuaRectClass

    --

    global world : BevyAPI.LuaWorld

    --

    global script : BevyAPI.LuaScriptData

    --

    global entity : BevyAPI.LuaEntity

    global log : BevyAPI.LogModule


return BevyAPI