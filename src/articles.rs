use patternfly_yew::*;
use yew::prelude::*;

#[function_component(Article1)]
pub fn article1() -> Html {
    html!{
        <Grid gutter=true>
            <GridItem cols={[12]} />
            <GridItem cols={[3]} />
            <GridItem cols={[6]}>
                <div>
                        <Text component={TextVariant::H2}>
                        {"Article Header"}
                        </Text>
                        <Text component={TextVariant::P}>{
                        "Lorem ipsum dolor sit amet, consectetur adipiscing 
                        elit. Nulla accumsan, metus ultrices eleifend gravida, nulla nunc varius lectus, nec 
                        rutrum justo nibh eu lectus. Ut vulputate semper dui. Fusce erat odio, sollicitudin 
                        vel erat vel, interdum mattis neque. Sub works as well!"
                    }</Text>
                        <Text component={TextVariant::P}>
                        {"Quisque ante lacus, malesuada ac auctor vitae, congue"}
                        <Text component={TextVariant::A}>{"non ante"}</Text>
                        {
                            ". Phasellus lacus ex, semper ac tortor nec, fringilla condimentum orci. Fusce eu 
                            rutrum tellus."
                        }
                    </Text>
                        <Text component={TextVariant::Blockquote}>{
                        "Ut venenatis, nisl scelerisque sollicitudin 
                        fermentum, quam libero hendrerit ipsum, ut blandit est tellus sit amet turpis."
                    }</Text>
                        <Text component={TextVariant::Small}>
                        {"Sometimes you need small text to display things like date created"}
                    </Text>
                    <div class="pf-c-code-editor">
                        <div class="pf-c-code-editor__main">
                            <div class="pf-c-code-editor__code">
                                <pre class="pf-c-code-editor__code-pre">
                                    {"// Code Example"}
                                </pre>
                            </div>
                        </div>
                    </div>
                </div>
            </GridItem>
            <GridItem cols={[3]} />
        </Grid>
    }
}


#[function_component(Article2)]
pub fn article2() -> Html {
    html!{
        <Grid gutter=true>
            <GridItem cols={[12]} />
            <GridItem cols={[3]} />
            <GridItem cols={[6]}>
                <div>
                        <Text component={TextVariant::H2}>
                        {"Article Header"}
                        </Text>
                        <Text component={TextVariant::P}>{
                        "Lorem ipsum dolor sit amet, consectetur adipiscing 
                        elit. Nulla accumsan, metus ultrices eleifend gravida, nulla nunc varius lectus, nec 
                        rutrum justo nibh eu lectus. Ut vulputate semper dui. Fusce erat odio, sollicitudin 
                        vel erat vel, interdum mattis neque. Sub works as well!"
                    }</Text>
                        <Text component={TextVariant::P}>
                        {"Quisque ante lacus, malesuada ac auctor vitae, congue"}
                        <Text component={TextVariant::A}>{"non ante"}</Text>
                        {
                            ". Phasellus lacus ex, semper ac tortor nec, fringilla condimentum orci. Fusce eu 
                            rutrum tellus."
                        }
                    </Text>
                        <Text component={TextVariant::Blockquote}>{
                        "Ut venenatis, nisl scelerisque sollicitudin 
                        fermentum, quam libero hendrerit ipsum, ut blandit est tellus sit amet turpis."
                    }</Text>
                        <Text component={TextVariant::Small}>
                        {"Sometimes you need small text to display things like date created"}
                    </Text>
                    <div class="pf-c-code-editor">
                        <div class="pf-c-code-editor__main">
                            <div class="pf-c-code-editor__code">
                                <pre class="pf-c-code-editor__code-pre">
                                    {"// Code Example"}
                                </pre>
                            </div>
                        </div>
                    </div>
                </div>
            </GridItem>
            <GridItem cols={[3]} />
        </Grid>
    }
}